_:

{
  projectRootFile = "flake.nix";
  programs = {
    deadnix.enable = true;
    leptosfmt.enable = true;
    nixfmt.enable = true;
    prettier.enable = true;
    statix.enable = true;
    taplo.enable = true;
  };
  settings.formatter.leptosfmt = {
    max_width = 65;
    tab_spaces = 4;
  };
  settings.global.excludes = [
    ".envrc"
    "*.png"
    "*.jpg"
    "*.ico"
  ];
}
