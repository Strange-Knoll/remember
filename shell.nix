{pkgs ? import <nixpkgs>{}}:
with pkgs;
mkShell rec {
  nativeBuildInputs = [
    cargo
    clippy
    gcc
    rustc
    rustfmt
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
