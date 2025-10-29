{ self, lib, crane }:
{
  default = self.overlays.wangyun;

  wangyun = final: prev:
    import ./default.nix {
      inherit lib;
      inherit (final) myLib;
    };

  libs = final: prev: rec {
    myLib = import ./lib.nix { inherit self lib; pkgs = final; };
    craneLib = (crane.mkLib prev).overrideToolchain myLib.rustToolchain;
  };
}
