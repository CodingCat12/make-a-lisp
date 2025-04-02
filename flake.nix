{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }@inputs: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [inputs.fenix.overlays.default];
    };
  in {
    devShells."${system}".default = pkgs.mkShell {
      packages = with pkgs; [
        (fenix.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
        ])
        rust-analyzer-nightly

        alejandra
        nil

        marksman
        markdownlint-cli2
      ];
    };

    packages.${system}.default = let
      toolchain = pkgs.fenix.minimal.toolchain;
    in
      (pkgs.makeRustPlatform {
        cargo = toolchain;
        rustc = toolchain;
      })
      .buildRustPackage {
        pname = "lisp";
        version = "0.1.0";

        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;
      };
  };
}
