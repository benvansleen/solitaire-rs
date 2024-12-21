{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
      flake-utils,
      treefmt-nix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;

        toolchain =
          with fenix.packages.${system};
          combine [
            latest.toolchain
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        pkg = pkgs.callPackage ./package.nix { inherit self craneLib; };

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
      in
      {
        packages.default = pkg.site-server;

        inherit (pkg) checks;
        formatter = treefmtEval.config.build.wrapper;

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = with pkgs; [ ];
        };
      }
    );
}
