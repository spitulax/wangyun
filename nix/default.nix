{ lib
, myLib
}:
{
  wangyun = myLib.mkCrate {
    pname = "wangyun";

    meta = {
      description = "Chinese character lookup tool with data from Wiktionary.";
      mainProgram = "wangyun";
      homepage = "https://github.com/spitulax/wangyun";
      license = lib.licenses.mit;
      platforms = lib.platforms.all;
    };
  };
}
