{
  description = "mini Lua development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
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
        devShells.default = pkgs.mkShell {
          name = "lua51";
          package = with pkgs; [
            lua51Packages.lua
          ];
        };
        devShells.lua52 = pkgs.mkShell {
          name = "lua52";
          package = with pkgs; [
            lua52Packages.lua
          ];
        };
        devShells.lua53 = pkgs.mkShell {
          name = "lua53";
          package = with pkgs; [
            lua53Packages.lua
          ];
        };
        devShells.lua54 = pkgs.mkShell {
          name = "lua54";
          package = with pkgs; [
            lua53Packages.lua
          ];
        };
      }
    );
}
