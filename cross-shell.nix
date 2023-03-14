# A simplest nix shell file with the project dependencies and
# a cross-compilation support.
{ pkgs }:
pkgs.mkShell rec {
  # Native project dependencies like build utilities and additional routines
  # like container building, linters, etc.
  nativeBuildInputs = with pkgs.pkgsBuildHost; [
    # Rust
    (rust-bin.stable.latest.minimal.override {
      extensions = [ "rustfmt" "clippy" "llvm-tools-preview" "rust-src" ];
    })

    # Will add some dependencies like libiconv
    rustBuildHostDependencies

    # Crate dependencies
    cargoDeps.openssl-sys
    protobuf # required by libp2p
  ];
  # Libraries essential to build the service binaries
  buildInputs = with pkgs; [
    # Enable Rust cross-compilation support
    rustCrossHook
  ];
  # Runtime dependencies that should be in the service container
  propagatedBuildInputs = with pkgs; [
    openssl.dev
  ];

  /* Service docker image definition

    To compile docker image run the following commands:

    ```shell
    # Setup the Nix cross compilation
    export NIX_CROSS_SYSTEM='{ config = "x86_64-unknown-linux-musl"; isStatic = false; useLLVM = true; }'
    # Compile cargo binary
    nix-shell --pure --arg crossSystem "$NIX_CROSS_SYSTEM" --run "cargo build --release"
    # Build docker image from the compiled service
    docker load <$(nix-build ./shell.nix -A dockerImage --arg crossSystem "$NIX_CROSS_SYSTEM")
    ```
  */
  passthru.dockerImage =
    {
      # Cargo workspace member name
      name ? "sequencer"
    , tag ? "latest"
    }:
    pkgs.pkgsBuildHost.dockerTools.buildLayeredImage {
      inherit tag name;

      contents = with pkgs; [
        coreutils
        bashInteractive
        dockerTools.caCertificates
        # Actual service binary compiled by Cargo
        (copyBinaryFromCargoBuild {
          inherit name;
          targetDir = ./target;
          buildInputs = propagatedBuildInputs;
        })
        # Utilites like ldd to help image debugging
        stdenv.cc.libc_bin
      ];

      config = {
        Cmd = [ name ];
        WorkingDir = "/";
        Expose = 8080;
      };
    };
}
