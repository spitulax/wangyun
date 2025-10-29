{
  description = "Chinese character lookup tool with data from Wiktionary.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, rust-overlay, ... }:
    let
      inherit (nixpkgs) lib;
      systems = [ "x86_64-linux" "aarch64-linux" ];
      eachSystem = f: lib.genAttrs systems f;
      pkgsFor = eachSystem
        (system:
          import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
              self.overlays.libs
              self.overlays.default
            ];
          });
    in
    {
      overlays = import ./nix/overlays.nix { inherit self lib crane; };

      packages = eachSystem (system:
        let
          pkgs = pkgsFor.${system};
        in
        {
          default = self.packages.${system}.wangyun;
          inherit (pkgs) wangyun;
        });

      devShells = eachSystem (system:
        let
          pkgs = pkgsFor.${system};
        in
        {
          default = pkgs.callPackage ./nix/shell.nix { };
        });
    };
}
