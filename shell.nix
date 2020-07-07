{ pkgs ? import <nixpkgs> {}
, lib ? pkgs.stdenv.lib
, mkShell ? pkgs.mkShell
}:

let
  foo = null;

in mkShell rec {
  nativeBuildInputs = (with pkgs; [
    pkgconfig
  ]);

  buildInputs = (with pkgs; [
    cargo
    rustc
    rust-analyzer
    xlibs.libX11.dev
    alsaLib
    alsaLib.dev
    cmake
    python3
    freetype
    toybox
    freetype
    expat
    vulkan-validation-layers
  ]);

  APPEND_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
    vulkan-loader
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
  ]);

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
  '';
}
