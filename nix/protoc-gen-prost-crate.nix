{pkgs ? import <nixpkgs> {}, ...}:
with pkgs;
let version = "0.1.6";
in rustPlatform.buildRustPackage {
  name = "protoc-gen-prost-crate";

  src = "${fetchFromGitHub {
    owner = "neoeinstein";
    repo = "protoc-gen-prost";
    sha256 = "sha256-WjgbrLtcIX4hEJJevHd7Ib4tb2BAbTSG9mOurqLcLxk=";
    rev = "protoc-gen-prost-crate-v${version}";
  }}";

  nativeBuildInputs = [cmake];

  buildAndTestSubdir = "protoc-gen-prost-crate";

  cargoSha256 = "sha256-C773H4BJ+ba33rIspDpdne3SLDK1pQ9Uw0iHytNXiWc=";
}
