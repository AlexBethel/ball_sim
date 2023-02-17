{ pkgs ? import <nixpkgs> {} }:
with pkgs;

mkShell {
  nativeBuildInputs = [
    SDL2.dev
  ];
}
