{
  description = "A very basic flake";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; 
    fenix.url = "github:nix-community/fenix";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        config,
        system,
        ...
      }: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.fenix.overlays.default
          ];
        };
      in {
        packages.default = pkgs.callPackage ./nix/default.nix {inherit pkgs;};
        devShells.default = pkgs.callPackage ./nix/shell.nix {inherit pkgs;};
        formatter = "${pkgs.alejandra}/bin/alejandra}";
      };
    };
}
