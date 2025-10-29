{ pkgs
, myLib
, craneLib
, rust-analyzer
, cargo-nextest
}:
craneLib.devShell ({
  name = "wangyun-shell";
  buildInputs = [
    (myLib.rustToolchain pkgs)
    rust-analyzer
    cargo-nextest
  ];
  inputsFrom = [
    (myLib.cargoArtifacts.overrideAttrs {
      cargoVendorDir = null;
    })
  ];
} // myLib.env)
