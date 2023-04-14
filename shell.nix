{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [
    pkgs.nodejs
    pkgs.yarn
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
  ];
}
