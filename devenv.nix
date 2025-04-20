{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.LIBCLANG_PATH = "${pkgs.libclang.lib}";
  env.OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

  # https://devenv.sh/packages/
  packages = [ pkgs.git pkgs.libclang.lib pkgs.openssl.dev ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "llvm-tools" "rustfmt" "rust-analyzer" ];
  };
}
