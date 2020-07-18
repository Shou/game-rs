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
    python3
    xlibs.libxcb
    xlibs.libX11.dev
    xlibs.libXcursor
    xlibs.libXrandr
    xlibs.libXi
    wayland
    wayland-protocols
    libGL
    libGLU
    libglvnd
  ]);

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
