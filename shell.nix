{
  pkgs ? import (import ./nix/sources.nix).nixpkgs {}
}:

let
  updateCrateDeps = pkgs.writeScriptBin "update-crate-deps" ''
    #!/bin/sh
    # We need recent patches due to the crate renaming feature
    nix run -f ${(import ./nix/sources.nix).crate2nix} -c crate2nix generate -n "<nixpkgs>" -f ./Cargo.toml -o Cargo.nix
  '';
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      rustfmt
      updateCrateDeps
      pkg-config
      rustc
      cargo
    ];
  }
