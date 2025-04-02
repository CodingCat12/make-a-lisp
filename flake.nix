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
    fenix,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
  in {
    devShells."${system}".default = pkgs.mkShell {
      packages = with pkgs; [
        rustc
        cargo
        rustfmt
        rust-analyzer
        clippy

        alejandra
        nil

        marksman
        markdownlint-cli2
      ];
    };

    packages.${system}.default = let
      toolchain = fenix.packages.${system}.minimal.toolchain;
      pkgs = nixpkgs.legacyPackages.${system};
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
