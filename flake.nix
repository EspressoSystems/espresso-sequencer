{
  description = "Espresso Decentralized Sequencer";

  nixConfig = {
    extra-substituters = [ "https://espresso-systems-private.cachix.org" ];
    extra-trusted-public-keys = [ "espresso-systems-private.cachix.org-1:LHYk03zKQCeZ4dvg3NctyCq88e44oBZVug5LpYKjPRI=" ];
  };

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.flake-compat.url = "github:edolstra/flake-compat";
  inputs.flake-compat.flake = false;

  inputs.pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";

  outputs = { self, nixpkgs, rust-overlay, flake-utils, flake-compat, pre-commit-hooks, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              doc = {
                enable = true;
                description = "Generate figures";
                entry = "make doc";
                types_or = [ "plantuml" ];
                pass_filenames = false;
              };
              cargo-fmt = {
                enable = true;
                description = "Enforce rustfmt";
                entry = "cargo fmt --all";
                pass_filenames = false;
              };
              cargo-sort = {
                enable = true;
                description = "Ensure Cargo.toml are sorted";
                entry = "cargo sort -g -w";
                pass_filenames = false;
              };
              cargo-clippy = {
                enable = true;
                description = "Run clippy";
                entry = "cargo clippy --workspace --examples --bins --tests -- -D warnings";
                pass_filenames = false;
              };
            };
          };
        };
        devShells.default =
          let
            stableToolchain = pkgs.rust-bin.stable.latest.minimal.override {
              extensions = [ "rustfmt" "clippy" "llvm-tools-preview" "rust-src" ];
            };
            # nixWithFlakes allows pre v2.4 nix installations to use
            # flake commands (like `nix flake update`)
            nixWithFlakes = pkgs.writeShellScriptBin "nix" ''
              exec ${pkgs.nixFlakes}/bin/nix --experimental-features "nix-command flakes" "$@"
            '';
          in
          mkShell
            {
              buildInputs = [
                # Rust dependencies
                pkgconfig
                openssl
                curl
                protobuf # to compile libp2p-autonat
                stableToolchain

                # Rust tools
                cargo-edit
                cargo-watch
                cargo-sort

                # Tools
                nixWithFlakes
                entr

                # Figures
                graphviz
                plantuml
                coreutils
              ];
              shellHook = ''
                # Prevent cargo aliases from using programs in `~/.cargo` to avoid conflicts
                # with rustup installations.
                export CARGO_HOME=$HOME/.cargo-nix
              '' + self.checks.${system}.pre-commit-check.shellHook;
              RUST_SRC_PATH = "${stableToolchain}/lib/rustlib/src/rust/library";
              RUST_BACKTRACE = 1;
              RUST_LOG = "info,libp2p=off";
            };
      }
    );
}
