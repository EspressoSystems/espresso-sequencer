{
  description = "Espresso Decentralized Sequencer";

  nixConfig = {
    extra-substituters = [
      "https://espresso-systems-private.cachix.org"
      "https://nixpkgs-cross-overlay.cachix.org"
    ];
    extra-trusted-public-keys = [
      "espresso-systems-private.cachix.org-1:LHYk03zKQCeZ4dvg3NctyCq88e44oBZVug5LpYKjPRI="
      "nixpkgs-cross-overlay.cachix.org-1:TjKExGN4ys960TlsGqNOI/NBdoz2Jdr2ow1VybWV5JM="
    ];
  };

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.nixpkgs-cross-overlay.url =
    "github:alekseysidorov/nixpkgs-cross-overlay";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.foundry.url =
    "github:shazow/foundry.nix/monthly"; # Use monthly branch for permanent releases
  inputs.solc-bin.url = "github:EspressoSystems/nix-solc-bin";

  inputs.flake-compat.url = "github:edolstra/flake-compat";
  inputs.flake-compat.flake = false;

  inputs.pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    , nixpkgs-cross-overlay
    , flake-utils
    , pre-commit-hooks
    , fenix
    , foundry
    , solc-bin
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      # node=error: disable noisy anvil output
      RUST_LOG = "info,libp2p=off,isahc=error,surf=error,node=error";
      RUST_BACKTRACE = 1;
      ASYNC_FLAGS =
        " --cfg async_executor_impl=\"async-std\" --cfg async_channel_impl=\"async-std\" ";
      RUSTFLAGS = "${ASYNC_FLAGS} --cfg hotshot_example";
      RUSTDOCFLAGS = ASYNC_FLAGS;
      # Use a distinct target dir for builds from within nix shells.
      CARGO_TARGET_DIR = "target/nix";
      rustEnvVars = {
        inherit RUST_LOG RUST_BACKTRACE RUSTFLAGS RUSTDOCFLAGS
          CARGO_TARGET_DIR;
      };

      solhintPkg = { buildNpmPackage, fetchFromGitHub }:
        buildNpmPackage rec {
          pname = "solhint";
          version = "3.6.2";
          src = fetchFromGitHub {
            owner = "protofire";
            repo = pname;
            rev = "refs/tags/${version}";
            hash = "sha256-VI6J2iSgimcT9TWPlPD6aIDfRFmlQafCc/J4dwF9rMs=";
          };
          npmDepsHash = "sha256-lSe3Rt3I2yFy9Je3SLD2QJA/608ppvbLWmwDt6vkDIk=";
          dontNpmBuild = true;
        };

      overlays = [
        (import rust-overlay)
        foundry.overlay
        solc-bin.overlays.default
        (final: prev: {
          solhint =
            solhintPkg { inherit (prev) buildNpmPackage fetchFromGitHub; };
        })
      ];
      pkgs = import nixpkgs { inherit system overlays; };
      crossShell = { config }:
        let
          localSystem = system;
          crossSystem = {
            inherit config;
            useLLVM = true;
            isStatic = true;
          };
          pkgs = import "${nixpkgs-cross-overlay}/utils/nixpkgs.nix" {
            inherit overlays localSystem crossSystem;
          };
        in
        import ./cross-shell.nix {
          inherit pkgs;
          envVars = rustEnvVars;
        };
    in
    with pkgs; {
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
              types_or = [ "rust" "toml" ];
              pass_filenames = false;
            };
            cargo-sort = {
              enable = true;
              description = "Ensure Cargo.toml are sorted";
              entry = "cargo sort -g -w";
              types_or = [ "toml" ];
              pass_filenames = false;
            };
            cargo-clippy = {
              enable = true;
              description = "Run clippy";
              entry = "just clippy";
              types_or = [ "rust" "toml" ];
              pass_filenames = false;
            };
            forge-fmt = {
              enable = true;
              description = "Enforce forge fmt";
              entry = "forge fmt";
              types_or = [ "solidity" ];
              pass_filenames = false;
            };
            solhint = {
              enable = true;
              description = "Solidity linter";
              entry = "solhint --fix 'contracts/{script,src,test}/**/*.sol'";
              types_or = [ "solidity" ];
              pass_filenames = true;
            };
            gas-report = {
              enable = true;
              description = "Update gas benchmark in .gas-snapshot";
              entry = "just gas-benchmarks";
              pass_filenames = false;
            };
            contract-bindings = {
              enable = true;
              description = "Generate contract bindings";
              entry = "just gen-bindings";
              types_or = [ "solidity" ];
              pass_filenames = false;
            };
            prettier-fmt = {
              enable = true;
              description = "Enforce markdown formatting";
              entry = "prettier -w";
              types_or = [ "markdown" "ts" ];
              pass_filenames = true;
            };
            spell-checking = {
              enable = true;
              description = "Spell checking";
              # --force-exclude to exclude excluded files if they are passed as arguments
              entry = "typos --force-exclude";
              pass_filenames = true;
              # Add excludes to the .typos.toml file instead
            };
            nixpkgs-fmt.enable = true;
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
        mkShell (rustEnvVars // {
          buildInputs = [
            # Rust dependencies
            pkg-config
            openssl
            curl
            protobuf # to compile libp2p-autonat
            stableToolchain
            jq

            # Rust tools
            cargo-audit
            cargo-edit
            cargo-sort
            typos
            just
            fenix.packages.${system}.rust-analyzer

            # Tools
            nixWithFlakes
            nixpkgs-fmt
            entr
            process-compose
            # `postgresql` defaults to an older version (15), so we select the latest version (16)
            # explicitly.
            postgresql_16

            # Figures
            graphviz
            plantuml
            coreutils

            # Ethereum contracts, solidity, ...
            foundry-bin
            solc
            nodePackages.prettier
            solhint
            (python3.withPackages (ps: with ps; [ black ]))
            yarn
          ] ++ lib.optionals stdenv.isDarwin
            [ darwin.apple_sdk.frameworks.SystemConfiguration ]
          ++ lib.optionals (!stdenv.isDarwin) [ cargo-watch ] # broken on OSX
          ;
          shellHook = ''
            # Add node binaries to PATH
            export PATH="$PWD/node_modules/.bin:$PATH"
            # Prevent cargo aliases from using programs in `~/.cargo` to avoid conflicts
            # with rustup installations.
            export CARGO_HOME=$HOME/.cargo-nix
            export PATH="$PWD/$CARGO_TARGET_DIR/release:$PATH"
          '' + self.checks.${system}.pre-commit-check.shellHook;
          RUST_SRC_PATH = "${stableToolchain}/lib/rustlib/src/rust/library";
          FOUNDRY_SOLC = "${solc}/bin/solc";
        });
      devShells.crossShell =
        crossShell { config = "x86_64-unknown-linux-musl"; };
      devShells.armCrossShell =
        crossShell { config = "aarch64-unknown-linux-musl"; };
      devShells.nightly =
        let
          toolchain = pkgs.rust-bin.nightly.latest.minimal.override {
            extensions = [ "rustfmt" "clippy" "llvm-tools-preview" "rust-src" ];
          };
        in
        mkShell (rustEnvVars // {
          buildInputs = [
            # Rust dependencies
            pkg-config
            openssl
            curl
            protobuf # to compile libp2p-autonat
            toolchain
          ];
        });
      devShells.coverage =
        let toolchain = pkgs.rust-bin.nightly.latest.minimal;
        in mkShell (rustEnvVars // {
          buildInputs = [
            # Rust dependencies
            pkg-config
            openssl
            curl
            protobuf # to compile libp2p-autonat
            toolchain
            grcov
          ];
          CARGO_INCREMENTAL = "0";
          shellHook = ''
            RUSTFLAGS="$RUSTFLAGS -Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests -Cdebuginfo=2"
          '';
          RUSTDOCFLAGS =
            "-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests";
        });

      devShells.rustShell =
        let
          stableToolchain = pkgs.rust-bin.stable.latest.minimal.override {
            extensions = [ "rustfmt" "clippy" "llvm-tools-preview" "rust-src" ];
          };
        in
        mkShell (rustEnvVars // {
          buildInputs = [
            # Rust dependencies
            pkg-config
            openssl
            curl
            protobuf # to compile libp2p-autonat
            stableToolchain
          ];
        });
    });
}
