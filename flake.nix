{
  description = "Generate directory thumbnails from images inside them";
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: with nixpkgs.legacyPackages.x86_64-linux; {
    packages.x86_64-linux.default = buildGoModule {
      name = "thud";
      src = ./.;

      buildInputs = with pkgs; [ vips ];
      nativeBuildInputs = with pkgs; [ pkg-config ];

      vendorSha256 = "sha256-NuosYDNDiygPArKLYrLg4jEjQhTumdZ/5GDy4JY4F9s=";

      postInstall = ''
        mkdir -p $out/share/thumbnailers
        substituteAll ${./thud.thumbnailer} $out/share/thumbnailers/thud.thumbnailer
      '';
    };

    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [ vips ];
      nativeBuildInputs = with pkgs; [ pkg-config ];
    };
  };
}
