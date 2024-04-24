use globset::Glob;
use ignore::WalkBuilder;
use std::process::{exit, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone)]
struct Config {
    help: bool,
    version: bool,
    verbose: bool,
}

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");
    let mut args = std::env::args();
    let executable = args.next().unwrap();

    let usage = format!(
        "
        Usage: {executable} [options] <path> <glob> <formatter> [<formatter-options>...]
        Options:
            -h, --help      Print help message.
            -V, --version   Print version information.
            -v, --verbose   Print verbose output.
    "
    );

    // parse arguments
    let mut config: Config = Config {
        help: false,
        version: false,
        verbose: false,
    };
    let mut positionals_found = false;
    let mut options = Vec::new();
    let mut positionals = Vec::new();
    for arg in args {
        if positionals_found {
            positionals.push(arg);
            continue;
        } else {
            if arg.starts_with("-") {
                options.push(arg);
            } else {
                positionals_found = true;
                positionals.push(arg);
            }
        }
    }
    println!("options: {:?}", options);
    println!("positionals: {:?}", positionals);

    // handle options
    if options.contains(&"-h".to_string()) || options.contains(&"--help".to_string()) {
        config.help = true;
        println!("{}", usage);
        return;
    }
    if options.contains(&"-V".to_string()) || options.contains(&"--version".to_string()) {
        config.version = true;
        println!("{} v{}", NAME, VERSION);
        return;
    }
    if options.contains(&"-v".to_string()) || options.contains(&"--verbose".to_string()) {
        println!("{} v{}", NAME, VERSION);
        config.verbose = true;
    }

    // validate positionals
    if positionals.len() < 3 {
        println!("{}", usage);
        return;
    }
    let path = &positionals[0];
    let glob = &positionals[1];
    let formatter_executable = &positionals[2];
    let formatter_options = &positionals[3..];

    println!("path: {}", path);
    println!("glob: {}", glob);
    println!("formatter_executable: {}", formatter_executable);
    println!("formatter_options: {:?}", formatter_options);

    let glob = Glob::new(glob).expect("Failed to parse glob pattern");
    let glob_matcher = Arc::new(glob.compile_matcher());

    // Use a thread-safe flag to track if any errors occurred
    let has_errors = Arc::new(AtomicBool::new(false));

    // walk the path ignoring hidden and gitignore files
    WalkBuilder::new(path)
        .hidden(true)
        .git_ignore(true)
        .build_parallel()
        .run(|| {
            let has_errors = has_errors.clone();
            let glob_matcher = glob_matcher.clone();
            let formatter_executable = formatter_executable.clone();
            let formatter_options = formatter_options;
            let config = config.clone();

            Box::new(move |result| {
                match result {
                    Ok(entry) => {
                        if !glob_matcher.is_match(entry.path()) {
                            return ignore::WalkState::Continue;
                        }
                        if config.verbose {
                            println!("Found: {}", entry.path().display());
                        }
                        let output = Command::new(&formatter_executable)
                            .args(formatter_options)
                            .arg(entry.path().to_str().unwrap())
                            .output()
                            .expect("Failed to execute process");

                        if !output.status.success() {
                            println!(
                                "Process failed with status: {} (file: {})",
                                output.status,
                                entry.path().display()
                            );
                            has_errors.store(true, Ordering::SeqCst);
                        }
                    }
                    Err(err) => {
                        println!("ERROR: {}", err);
                        has_errors.store(true, Ordering::SeqCst);
                    }
                }
                ignore::WalkState::Continue
            })
        });

    if has_errors.load(Ordering::SeqCst) {
        println!("One or more processes failed.");
        exit(1);
    }
}
