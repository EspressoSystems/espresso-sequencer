{
  description = "espresso-evm";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.flake-compat.url = "github:edolstra/flake-compat";
  inputs.flake-compat.flake = false;

  outputs = { self, nixpkgs, rust-overlay, flake-utils, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell
          {
            buildInputs = [
              graphviz
              plantuml
              coreutils
            ];
          };
      }
    );
}
