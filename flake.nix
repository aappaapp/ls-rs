{
  description = "ls implemented in Rust";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem = { pkgs, ... }: {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
          ];
        };
        packages.default = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          pname = "ls-rs";
          version = "0.1.0";
          src = ./.;
          cargoHash = "sha256-e+QujEq7436eFvNpMIXU0Cb3NS0wHpqlSoWz5G9x66c=";
        });
      };
      flake = { };
    };
}
