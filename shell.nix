{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    bun
    nodejs
    typescript
    # TODO: add more
  ];

  shellHook = ''
    echo "Development environment for Project Apple is ready. Thanks for using Nix!!"
  '';
}
