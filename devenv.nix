{ pkgs, ... }:

rec {
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
      cargo
      cargo-watch
      grpcurl
      sccache
      openapi-generator-cli
      swagger-cli
      git
    ] ++ (lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
      CoreFoundation
      SystemConfiguration
      Security
    ]));

  # https://devenv.sh/languages/
  languages.nix.enable = true;
  languages.rust.enable = true;

  scripts.ci.exec = ''
    cargo run -p haqq-build
    cargo test --workspace
    
    git diff --exit-code
  '';
}
