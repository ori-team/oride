{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  pname = "oride";
  version = "0.2.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  meta = with pkgs.lib; {
    description = "Fast, modular and extensible terminal code editor and mini-IDE in Rust";
    homepage = "https://github.com/ori-team/oride";
    license = licenses.mit;
    mainProgram = "oride";
  };
}
