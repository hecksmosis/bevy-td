{
  description = "Bevy flake";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShell = with pkgs; mkShell
          rec {
            nativeBuildInputs = [
              lldb
              pkg-config
            ];

            buildInputs = [
              zstd
              alsa-lib
              libxkbcommon
              udev
              vulkan-loader
              wayland
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
      });
}
