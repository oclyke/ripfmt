# ripfmt
ripfmt recursively applies code formatting tools while respecting your gitignore

**status**

extremely early development.
co-contibutors extremely welcome!

**intent**
ripfmt wants to be like [`ripgrep`](https://github.com/BurntSushi/ripgrep) - fast and focused.

**scope**
ripfmt does not actually format any code.
this is a job that is better left to language-specific tools.

the point of ripfmt is to address a common pattern in consinuous integration:
enforcing code formatting standards across a codebase.

**vision**

no plugins.
`ripfmt` will handle finding the files and spawning highly parallel processes for your code formatting tools.

**long range**

eventually it would be nice to have some configurability.
for example, what if ripfmt wants to support a `--check` feature?
this is a highly desireable feature for a tool used in CI,
however every existing tool has a different invocation for this feature.

`black . --check`

conceptual idea for command line templating.
`ripfmt --check 'black {file} {check ? --check : ""}'`

this would allow no-plugin ripfmt to support generic tools.
the template could be parsed and validated once at the beginning of the run and then used to spawn processes in parallel.
the `--check` flag passed to ripgrep would inform error handling and exit code.

# packaging

thanks to the wonderful [cargo2nix](https://github.com/cargo2nix/cargo2nix) project `ripfmt` has a nix flake.

stuff is fairly straightforward.
on a suitable system (with nix and flakes and such) you can run `cargo2nix` to generate `Cargo.nix` from the `Cargo.toml` and `Cargo.lock` files.

from the root of this project (where the Cargo.toml and Cargo.lock files are) you can build and run the latest `cargo2nix` generator like so:
```
nix run github:cargo2nix/cargo2nix
```

if/when cargo2nix needs to be updated the `nix flake update` family of commands will come into play.
```
nix flake lock --update-input cargo2nix
```

