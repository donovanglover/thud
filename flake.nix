{
  description = "Generate directory thumbnails from images inside them.";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages = {
          default = ./.;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ go ];
        };
      });
}
