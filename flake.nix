{
  description = "Simple TUI audio mixer for PipeWire";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/26f4107e47e3ad0ffd0db220469ba83b6f6d96e8";
    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      ...
    }:
    let
      eachSystem =
        callback: nixpkgs.lib.genAttrs (import systems) (system: callback nixpkgs.legacyPackages.${system});
    in
    {
      devShells = eachSystem (pkgs: {
        default =
          with pkgs;
          mkShell {
            packages = [
              rustc
              cargo
              rustfmt
              nixfmt
              clippy
              pkg-config
              rustPlatform.bindgenHook
              typos

              pipewire
            ];
          };
      });

      apps = eachSystem (
        pkgs:
        let
          app = {
            type = "app";
            program = pkgs.lib.getExe self.packages.${pkgs.stdenv.hostPlatform.system}.wiremix;
            meta.description = "Launch the Wiremix audio mixer";
          };
        in
        {
          default = app;
          wiremix = app;
        }
      );

      checks = eachSystem (pkgs: {
        wiremix = self.packages.${pkgs.stdenv.hostPlatform.system}.wiremix;
      });

      formatter = eachSystem (pkgs: pkgs.nixfmt);

      packages = eachSystem (
        pkgs:
        let
          package = pkgs.callPackage ./package.nix { };
        in
        {
          default = package;
          wiremix = package;
        }
      );
    };
}
