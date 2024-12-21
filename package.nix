{
  self,
  pkgs,
  lib,
  craneLib,
}:

let
  crate = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  leptos-options = crate.package.metadata.leptos // {
    bin-name = crate.package.name;
    bin-features = lib.concatStringsSep " " crate.package.metadata.leptos.bin-features;
    lib-features = lib.concatStringsSep " " crate.package.metadata.leptos.lib-features;
  };

  src = lib.cleanSourceWith {
    src = self; # The original, unfiltered source
    filter =
      path: type:
      (lib.hasInfix "/public/" path)
      || (lib.hasInfix "/style/" path)
      || (craneLib.filterCargoSources path type);
  };
  common-args = {
    inherit src;

    pname = leptos-options.bin-name;

    doCheck = false;
    nativeBuildInputs =
      with pkgs;
      [
        mold # faster compilation
        binaryen # provides wasm-opt
      ]
      ++ lib.optionals stdenv.isDarwin [
        pkgs.libiconv # character encoding lib for darwin
      ];
  };

  site-frontend-deps = craneLib.mkCargoDerivation (
    common-args
    // {
      pname = "site-frontend-deps";
      src = craneLib.mkDummySrc common-args;
      cargoArtifacts = null;
      doInstallCargoArtifacts = true;

      buildPhaseCargoCommand = ''
        cargo build \
          --features ${leptos-options.lib-features} \
          --lib \
          --target-dir=/build/source/target/front \
          --target=wasm32-unknown-unknown \
          --no-default-features \
          --profile=${leptos-options.lib-profile-release}
      '';
    }
  );

  site-server-deps = craneLib.mkCargoDerivation (
    common-args
    // {
      pname = "site-server-deps";
      src = craneLib.mkDummySrc common-args;
      cargoArtifacts = site-frontend-deps;
      doInstallCargoArtifacts = true;

      buildPhaseCargoCommand = ''
        cargo build \
          --features ${leptos-options.bin-features} \
          --no-default-features \
          --release
      '';
    }
  );

in
rec {
  site-server = craneLib.buildPackage (
    common-args
    // {
      cargoArtifacts = site-server-deps;
      nativeBuildInputs =
        common-args.nativeBuildInputs
        ++ (with pkgs; [
          cargo-leptos
          makeWrapper
        ]);

      buildPhaseCargoCommand = ''
        cargo leptos build --release -vvv
      '';
      installPhaseCommand = ''
        mkdir -p $out/bin
        cp target/release/${leptos-options.bin-name} $out/bin/
        cp -r target/site $out/bin/
        wrapProgram $out/bin/${leptos-options.bin-name} \
          --set LEPTOS_SITE_ROOT $out/bin/site
      '';

    }
  );

  checks = {
    # Build the crate as part of `nix flake check` for convenience
    inherit site-server;

    site-server-clippy = craneLib.cargoClippy (
      common-args
      // {
        cargoArtifacts = site-server-deps;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      }
    );
  };
}
