{ pkgs ? import (import ./nix/sources.nix).nixpkgs {} }:

with pkgs;

let
  # TODO: fix gitignore and replace this by gitignre
  # See https://github.com/NixOS/nixpkgs/issues/69138
  sources = builtins.path {
    name = "hydra-cli-filtered-source";
    path = ./.;
    filter = (path: type:
      baseNameOf path != ".git" &&
      baseNameOf path != "default.nix" &&
      baseNameOf path != "target" &&
      baseNameOf path != "result" &&
      (! (pkgs.lib.hasSuffix ".rs.bk" path)) &&
      (! (pkgs.lib.hasSuffix "~" path))
    );
  };
in
((callPackage ./Cargo.nix {}).rootCrate.build).overrideDerivation(_: {
  src = sources;
  doCheck = true;
  checkPhase = ''
    echo "Checking formatting with 'rustfmt'"
    find . -name "*.rs" | xargs ${rustfmt}/bin/rustfmt --check
  '';
})
