{pkgs}: let
  toolchain = pkgs.fenix.minimal.toolchain;
in
  (pkgs.makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  })
  .buildRustPackage {
    pname = "lisp";
    version = "0.1.0";

    src = ../.;

    cargoLock.lockFile = ../Cargo.lock;
  }
