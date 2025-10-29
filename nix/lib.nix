{ self
, pkgs
, lib
}:
let
  inherit (pkgs) craneLib;
in
rec {
  rustToolchain = pkgs: pkgs.rust-bin.stable.latest.default;

  mkDate = longDate: (lib.concatStringsSep "-" [
    (builtins.substring 0 4 longDate)
    (builtins.substring 4 2 longDate)
    (builtins.substring 6 2 longDate)
  ]);

  src = lib.cleanSource ./..;

  version = src: (craneLib.crateNameFromCargoToml { inherit src; }).version
    + "+date=" + (mkDate (self.lastModifiedDate or "19700101"))
    + "_" + (self.shortRev or "dirty");

  env = {
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };

  commonArgs = {
    inherit src;
    version = version src;
    strictDeps = true;
    nativeBuildInputs = with pkgs; [
      openssl.dev
    ];
    buildInputs = [ ];
  } // env;

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  mkCrate =
    { pname
    , ...
    }@args:
    craneLib.buildPackage (commonArgs // {
      inherit pname cargoArtifacts;
      doCheck = false;
      cargoExtraArgs = "-p ${pname}";
    } // args);
}
