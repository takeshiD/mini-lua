{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          lua51 = pkgs.lua51Packages.lua;
          lua52 = pkgs.lua52Packages.lua;
          lua53 = pkgs.lua53Packages.lua;
          lua54 = pkgs.lua54Packages.lua;
        };
        devShells = {
          lua51 = pkgs.mkShell {
            name = "lua51";
            packages = with pkgs; [
              lua51Packages.lua
            ];
          };
          lua52 = pkgs.mkShell {
            name = "lua52";
            packages = with pkgs; [
              lua52Packages.lua
            ];
          };
          lua53 = pkgs.mkShell {
            name = "lua53";
            packages = with pkgs; [
              lua53Packages.lua
            ];
          };
          lua54 = pkgs.mkShell {
            name = "lua54";
            packages = with pkgs; [
              lua54Packages.lua
            ];
          };
        };
      }
    );
}
