{ ... }:

{
  projectRootFile = "flake.nix";
  programs = {
    deadnix.enable = true;
    leptosfmt.enable = true;
    nixfmt.enable = true;
    prettier.enable = true;
    taplo.enable = true;
  };
  settings.global.excludes = [
    ".envrc"
    "*.png"
    "*.jpg"
    "*.ico"
  ];
}
