{
  description = "Espresso Decentralized Sequencer";

  nixConfig = {
    extra-substituters = [ "https://espresso-systems-private.cachix.org" ];
    extra-trusted-public-keys = [ "espresso-systems-private.cachix.org-1:LHYk03zKQCeZ4dvg3NctyCq88e44oBZVug5LpYKjPRI=" ];
  };

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.nixpkgs-cross-overlay = {
    url = "github:alekseysidorov/nixpkgs-cross-overlay";
    inputs.flake-utils.follows = "flake-utils";
  };

  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.foundry.url = "github:shazow/foundry.nix/monthly"; # Use monthly branch for permanent releases
  inputs.solc-bin.url = "github:EspressoSystems/nix-solc-bin";

  inputs.flake-compat.url = "github:edolstra/flake-compat";
  inputs.flake-compat.flake = false;

  inputs.pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";


  outputs = { self, nixpkgs, rust-overlay, nixpkgs-cross-overlay, flake-utils, flake-compat, pre-commit-hooks, fenix, foundry, solc-bin, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        info = builtins.split "\([a-zA-Z0-9_]+\)" system;
        arch = (builtins.elemAt (builtins.elemAt info 1) 0);
        os = (builtins.elemAt (builtins.elemAt info 3) 0);
        RUST_LOG = "info,libp2p=off,isahc=error,surf=error";
        overlays = [
          (import rust-overlay)
          foundry.overlay
          solc-bin.overlays.default
        ];
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
                entry = "cargo clippy --workspace --all-features --all-targets -- -D warnings";
                pass_filenames = false;
              };
              forge-fmt = {
                enable = true;
                description = "Enforce forge fmt";
                entry = "cd contracts && forge fmt --check";
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
            solc = pkgs.solc-bin.latest;
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
                cargo-audit
                cargo-edit
                cargo-watch
                cargo-sort
                just
                fenix.packages.${system}.rust-analyzer

                # Tools
                nixWithFlakes
                entr

                # Figures
                graphviz
                plantuml
                coreutils

                foundry-bin
                solc
              ] ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.SystemConfiguration ];
              shellHook = ''
                # Prevent cargo aliases from using programs in `~/.cargo` to avoid conflicts
                # with rustup installations.
                export CARGO_HOME=$HOME/.cargo-nix
              '' + self.checks.${system}.pre-commit-check.shellHook;
              RUST_SRC_PATH = "${stableToolchain}/lib/rustlib/src/rust/library";
              RUST_BACKTRACE = 1;
              inherit RUST_LOG;
              FOUNDRY_SOLC = "${solc}/bin/solc";
            };
        devShells.staticShell =
          let
            muslPkgs = import nixpkgs {
              localSystem = system;
              crossSystem = { config = "${arch}-unknown-${os}-musl"; };
            };
            stableMuslRustToolchain =
              pkgs.rust-bin.stable.latest.minimal.override {
                extensions = [ "rustfmt" "clippy" "llvm-tools-preview" "rust-src" ];
                targets = [ "${arch}-unknown-${os}-musl" ];
              };
            opensslMusl = muslPkgs.openssl.override { static = true; };
            curlMusl = (muslPkgs.pkgsStatic.curl.override {
              http2Support = false;
              libssh2 = muslPkgs.pkgsStatic.libssh2.dev;
              zstdSupport = false;
              idnSupport = false;
            }).overrideAttrs (oldAttrs:
              let confFlags = oldAttrs.configureFlags;
              in {
                configureFlags = (muslPkgs.lib.take 13 confFlags)
                  ++ (muslPkgs.lib.drop 14 confFlags)
                  ++ [ (muslPkgs.lib.withFeature true "libssh2") ];
              });
          in
          mkShell {
            DEP_CURL_STATIC = "y";
            "CARGO_TARGET_${pkgs.lib.toUpper arch}_UNKNOWN_${pkgs.lib.toUpper os}_MUSL_LINKER" =
              "${pkgs.llvmPackages_latest.lld}/bin/lld";
            RUSTFLAGS =
              "-C target-feature=+crt-static -L${opensslMusl.out}/lib/ -L${curlMusl.out}/lib -L${muslPkgs.pkgsStatic.zstd.out}/lib -L${muslPkgs.pkgsStatic.libssh2}/lib -L${muslPkgs.pkgsStatic.openssl}/lib -lssh2";
            OPENSSL_STATIC = "true";
            OPENSSL_DIR = "-L${muslPkgs.pkgsStatic.openssl}";
            OPENSSL_INCLUDE_DIR = "${opensslMusl.dev}/include/";
            OPENSSL_LIB_DIR = "${opensslMusl.dev}/lib/";
            CARGO_BUILD_TARGET = "${arch}-unknown-${os}-musl";
            buildInputs = with pkgs;
              [ protobuf stableMuslRustToolchain fd cmake ];
            meta.broken = if "${os}" == "darwin" then true else false;

            inherit RUST_LOG;
          };
        devShells.crossShell = import ./cross-shell.nix {
          localSystem = system;
          crossSystem = { config = "x86_64-unknown-linux-musl"; useLLVM = true; };
        };
      }
    );
}
