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
    ...
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [fenix.overlays.default];
    };
  in {
    packages.${system}.default = pkgs.callPackage ./nix/default.nix {inherit pkgs;};
    devShells."${system}".default = pkgs.callPackage ./nix/shell.nix {inherit pkgs;};
  };
}
