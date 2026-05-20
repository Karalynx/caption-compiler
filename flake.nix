{
  description = "Development environment for the project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        toolchain = (pkgs.lib.importTOML ./rust-toolchain.toml);
        rust = {
          version = pkgs.rust-bin.stable.${toolchain.channel};
          platform = pkgs.makeRustPlatform {
            cargo = rust.version.minimal;
            rustc = rust.version.minimal;
          };
        };
      in {
        packages.default = rust.platform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;

          src = pkgs.lib.cleanSource ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust.version.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
          ];

          RUST_BACKTRACE = 1;
        };
      }
    );
}