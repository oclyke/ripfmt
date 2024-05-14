{
  description = "ripfmt";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = { self, nixpkgs, cargo2nix, ... }@inputs:
    let
      systems =
        [ "x86_64-linux" "x86_64-darwin" "x86_64-windows" "aarch64-linux" "aarch64-darwin" ];
      lib = (import nixpkgs { system = "x86_64-linux"; }).lib;
    in {
      # Executed by `nix build .#<name>`
      # packages.<system>.<name> = derivation;
      packages = lib.genAttrs systems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ cargo2nix.overlays.default ];
          };

          rustPkgs = pkgs.rustBuilder.makePackageSet {
            rustVersion = "1.75.0";
            packageFun = import ./Cargo.nix;
          };

        in rec {
          ripfmt = (rustPkgs.workspace.ripfmt { });
          default = ripfmt;
        });
    };
}
