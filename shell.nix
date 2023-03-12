{ localSystem ? builtins.currentSystem
, crossSystem ? { config = "x86_64-unknown-linux-musl"; isStatic = true; useLLVM = true; }
}:
let
  # Fetch the nixpkgs-cross-overlay sources.
  src = builtins.fetchTarball "https://github.com/alekseysidorov/nixpkgs-cross-overlay/tarball/main";
  # Use the nixpkgs revision provided by the overlay. 
  # This is the best way, as they are the most proven and compatible.
  nixpkgs = "${src}/utils/nixpkgs.nix";
  # Make cross system packages.
  pkgs = import nixpkgs {
    inherit localSystem crossSystem;
    overlays = [
      # <- You may add your extra overlays here.
    ];
  };
in
# And now, with the resulting packages, we can describe the cross-compilation shell.
pkgs.mkShell {
  # Native project dependencies like build utilities and additional routines 
  # like container building, linters, etc.
  nativeBuildInputs = [
    pkgs.pkgsBuildHost.rust-bin.stable.latest.default
    # Will add some dependencies like libiconv.
    pkgs.pkgsBuildHost.rustBuildHostDependencies
    # Crates dependencies.
    pkgs.cargoDeps.openssl-sys
    pkgs.pkgsBuildHost.protobuf

    # A simple script to create a docker image from the Cargo workspace member.
    (pkgs.pkgsBuildHost.writeShellApplication {
      name = "cargo-nix-docker-image";
      runtimeInputs = with pkgs.pkgsBuildHost; [
        nix
        docker
      ];
      text = let shellFile = ./shell.nix; in ''
        binary_name=$1
        # Compile cargo binary
        cargo build --release
        # Copy this shell to the target dir
        cp ${shellFile} ./target/shell.nix
        # Build docker image from the compiled service
        image_archive=$(nix-build ./target/shell.nix -A dockerImage --argstr name "$binary_name")
        docker load <"$image_archive"
      '';
    })
  ];
  # Libraries essential to build the service binaries.
  buildInputs = with pkgs; [
    # Enable Rust cross-compilation support.
    rustCrossHook
  ]; 
  # propagatedBuildInputs = with pkgs; [ protobuf ];
  # Prettify shell prompt.
  shellHook = ''
    ${pkgs.crossBashPrompt}
    echo "Welcome to the Cargo docker images builder demo shell!"
    echo ""
    echo "Usage:"
    echo ""
    echo "$ cargo-nix-docker-image <executable-name>"
    echo ""
    echo "Have a nice day!"
  '';
  /* Service docker image definition

    Usage: 

    ```shell
    cargo-nix-docker-image executable-name>
    ```
  */
  passthru.dockerImage = (
    {
      # Cargo workspace member name
      name
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
          targetDir = ./.;
          buildInputs = [
            openssl.dev
          ];
        })
        # Utilites like ldd to help image debugging
        stdenv.cc.libc_bin
      ];

      config = {
        Cmd = [ name ];
        WorkingDir = "/";
        Expose = 8080;
      };
    }
  );
}
