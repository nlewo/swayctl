{

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk = {
      url = "github:nmattia/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk {};
      in rec {

        packages.swayctl = naersk-lib.buildPackage {
          root = ./swayctl;
          doCheck = true;
          cargoTestCommands = ts: ts ++ [
            ''find . -name main.rs | xargs ${pkgs.rustfmt}/bin/rustfmt --check''
          ];
        };

        defaultPackage = packages.swayctl;

        apps.swayctl = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
          exePath = "/bin/swayctl";
        };

        defaultApp = apps.swayctl;

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

      });

}
