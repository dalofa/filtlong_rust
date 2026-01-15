# let 
#     load_nixpkgs_unstable = import ( builtins.fetchTarball {
#         url = https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz;
#     }) {};
# in
# # Shell configuration (in NIX) for my personal termnial experience
# { pkgs ? load_nixpkgs_unstable, stdenv }:
## nixpkgs-unstable:
# Rolling releases follow master, the main development branch.
#     On Linux (including NixOS and WSL), use nixos-unstable.
#     On any other platform, use nixpkgs-unstable.
# *-small channel branches have passed a smaller test suite, which means they are more up-to-date with respect to their base branch, but offer fewer stability guarantees.
## [https://releases.nixos.org/nixpkgs/nixpkgs-25.05pre792306.70b191e2e0b1] [https://status.nixos.org/]
# 
{
  pkgs ? import <nixpkgs> { },
}:
let
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { };
in
############################################################
  ##  executing the script
  ############################################################
  ## Running with another shell than bash:
  ## non-flake:
  #   nix-shell --command zsh
  ## flake: 
  #   nix develop --command zsh
  ############################################################
  ##   defining shell packages
  ############################################################
  ## Can be defined through either 
  #   pkgs.mkShellNoCC { packages = ... }
  # or
  #   pkgs.mkShell { nativeBuildInputs = ... }
  pkgs.mkShellNoCC {
    packages = [
        (with fenix; (combine [
          ## makes a toolchain from the default profile with specified components ...
          minimal.toolchain
          # (default.withComponents [
          #   cargo rustc rust-src rustfmt clippy
          # ])
          targets.wasm32-unknown-unknown.latest.rust-std
          targets.x86_64-unknown-linux-musl.latest.rust-std
          # rust-analyzer
        ]))
        # fenix.rust-analyzer
        pkgs.gcc
    ];

    GREETING = "HEJ! B^)";

    shellHook = ''
      echo $GREETING
      codium .
    '';
  }
