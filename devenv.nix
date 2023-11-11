{ pkgs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";
  env.RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  # don't clash with rust-analyzer
  env.CARGO_TARGET_DIR = "target/t";

  dotenv.enable = true;

  # https://devenv.sh/packages/
  packages = with pkgs;
    [
      buf
      grpc
      cargo-watch
      grpcurl
      sccache
      openapi-generator-cli
      swagger-cli
    ] ++ (lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
      CoreFoundation
      SystemConfiguration
      Security
    ]));

  # https://devenv.sh/languages/
  languages.nix.enable = true;
  languages.rust.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
