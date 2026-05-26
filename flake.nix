{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:SpiralP/nix-flake-utils";
  };

  # build with `.?submodules=1`
  outputs = inputs@{ flake-utils, ... }:
    flake-utils.lib.makeOutputs inputs
      ({ lib, pkgs, makeRustPackage, ... }:
        let
          args = {
            src = lib.sourceByRegex ./. [
              "^\.cargo(/.*)?$"
              "^build\.rs$"
              "^Cargo\.(lock|toml)$"
              "^ClassiCube(/.*)?$"
              "^README\.md$"
              "^src(/.*)?$"
            ];

            nativeBuildInputs = with pkgs; [
              rustPlatform.bindgenHook
            ];
          };
        in
        {
          default = makeRustPackage pkgs (self: args);
        });
}
