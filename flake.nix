# Copyright (c) 2022 Espresso Systems (espressosys.com)
# This file is part of the HotShot Query Service library.
#
# This program is free software: you can redistribute it and/or modify it under the terms of the GNU
# General Public License as published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
# even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# General Public License for more details.
# You should have received a copy of the GNU General Public License along with this program. If not,
# see <https://www.gnu.org/licenses/>.

{
  description = "Generic query service for HotShot applications";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.flake-compat.url = "github:edolstra/flake-compat";
  inputs.flake-compat.flake = false;

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  inputs.pre-commit-hooks.inputs.flake-utils.follows = "flake-utils";
  inputs.pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, flake-utils, flake-compat, rust-overlay, fenix, pre-commit-hooks, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        info = builtins.split "\([a-zA-Z0-9_]+\)" system;
        arch = (builtins.elemAt (builtins.elemAt info 1) 0);
        os = (builtins.elemAt (builtins.elemAt info 3) 0);
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.minimal.override {
          extensions = [ "rustfmt" "clippy" "llvm-tools-preview" "rust-src" ];
        };
        rustDeps = with pkgs;
          [
            pkgconfig
            openssl
            bash

            curl

            cargo-edit
            cargo-udeps
            cargo-sort
            cmake
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.CoreFoundation
            darwin.apple_sdk.frameworks.SystemConfiguration

            # https://github.com/NixOS/nixpkgs/issues/126182
            libiconv
          ] ++ lib.optionals (stdenv.system != "aarch64-darwin") [
            cargo-watch # broken: https://github.com/NixOS/nixpkgs/issues/146349
          ];
        # nixWithFlakes allows pre v2.4 nix installations to use
        # flake commands (like `nix flake update`)
        nixWithFlakes = pkgs.writeShellScriptBin "nix" ''
          exec ${pkgs.nixFlakes}/bin/nix --experimental-features "nix-command flakes" "$@"
        '';
        pythonEnv = pkgs.poetry2nix.mkPoetryEnv { projectDir = ./.; };
        myPython = with pkgs; [ poetry pythonEnv ];
        shellHook  = ''
          # on mac os `bin/pwd -P` returns the canonical path on case insensitive file-systems
          my_pwd=$(/bin/pwd -P 2> /dev/null || pwd)

          export PATH=${pkgs.xdot}/bin:$PATH
          export PATH=''${my_pwd}/bin:$PATH
        '';
      in {
      	checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              cargo-fmt = {
                enable = true;
                description = "Enforce rustfmt";
                entry = "cargo fmt --all -- --check";
                pass_filenames = false;
              };
              cargo-sort = {
                enable = true;
                description = "Ensure Cargo.toml are sorted";
                entry = "cargo sort -g -w -c";
                pass_filenames = false;
              };
              cargo-clippy = {
                enable = true;
                description = "Run clippy";
                entry = "cargo clippy --workspace --all-features --all-targets -- -D clippy::dbg-macro";
                pass_filenames = false;
              };
              license-header-c-style = {
                enable = true;
                description =
                  "Ensure files with c-style comments have license header";
                entry = ''
                  insert_license --license-filepath .license-header.txt  --comment-style "//"'';
                types_or = [ "rust" ];
                pass_filenames = true;
              };
              license-header-hash = {
                enable = true;
                description =
                  "Ensure files with hash style comments have license header";
                entry = ''
                  insert_license --license-filepath .license-header.txt --comment-style "#"'';
                types_or = [ "bash" "python" "toml" "nix" ];
                excludes = [ "poetry.lock" ];
                pass_filenames = true;
              };
              license-header-html = {
                enable = true;
                description = "Ensure markdown files have license header";
                entry = ''
                  insert_license --license-filepath .license-header.txt --comment-style "<!--| ~| -->"'';
                types_or = [ "markdown" ];
                pass_filenames = true;
              };
            };
          };
        };
        devShell = pkgs.mkShell {
          shellHook = shellHook
          	# install pre-commit hooks
            + self.checks.${system}.pre-commit-check.shellHook;
          buildInputs = with pkgs;
            [
              fenix.packages.${system}.rust-analyzer
              nixWithFlakes
              nixpkgs-fmt
              git
              mdbook # make-doc, documentation generation
              rustToolchain
            ] ++ myPython ++ rustDeps;

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = 1;
          RUST_LOG = "info";
        };
      });
}
