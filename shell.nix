{
  pkgs ? import <nixpkgs> { },
}: let
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
in
pkgs.mkShell {
  packages = with pkgs; [
    rustup # rust but I'm lazy
  ];
}
