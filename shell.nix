{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ cargo clang cmake libfreefare libnfc rustfmt ];
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
