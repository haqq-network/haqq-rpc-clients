use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fs, io};

use regex::Regex;

const PROTO_GEN_DIR: &str = "src/gen";
const OPENAPI_SPEC_GEN_DIR: &str = "openapi";
const OPENAPI_VERSION: &str = "0.1.0";
const OPENAPI_GEN_DIR: &str = "rest";

fn main() {
    if std::path::Path::new(PROTO_GEN_DIR).exists() {
        fs::remove_dir_all(PROTO_GEN_DIR).unwrap();
    }

    let stdout = Stdio::inherit();
    let stderr = Stdio::inherit();

    let mut cmd = Command::new("buf");
    cmd.arg("generate");

    cmd.stdout(stdout).stderr(stderr);

    let output = cmd.output().unwrap();
    assert!(output.status.success());

    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/cosmos.msg.v1.rs", PROTO_GEN_DIR))
        .unwrap();

    f.write_all(b"// an empty stub, needed because prost-crate expects it to be there while prost doesn't generate the file").unwrap();
    std::mem::drop(f);

    apply_proto_patches(Path::new(PROTO_GEN_DIR));
    apply_openapi_patches(Path::new(OPENAPI_SPEC_GEN_DIR));

    let stdout = Stdio::inherit();
    let stderr = Stdio::inherit();

    if std::path::Path::new(OPENAPI_GEN_DIR).exists() {
        fs::remove_dir_all(OPENAPI_GEN_DIR).unwrap();
    }

    let mut cmd = Command::new("openapi-generator-cli");
    cmd.args([
        "generate",
        "-i",
        &format!("{}/apidocs.swagger.json", OPENAPI_SPEC_GEN_DIR),
        "-g",
        "rust",
        "-o",
        OPENAPI_GEN_DIR,
        "--skip-validate-spec",
        "--package-name",
        "haqq-rest",
    ]);

    cmd.stdout(stdout).stderr(stderr);

    let output = cmd.output().unwrap();
    assert!(output.status.success());
}

// https://github.com/cosmos/cosmos-rust/blob/08353cb090aea4ad1569c5962a1a92d6327cd2d1/proto-build/src/main.rs#L396
fn patch_file(path: impl AsRef<Path>, pattern: &Regex, replacement: &str) -> io::Result<()> {
    let mut contents = fs::read_to_string(&path)?;
    contents = pattern.replace_all(&contents, replacement).to_string();
    fs::write(path, &contents)
}

fn apply_openapi_patches(openapi_dir: &Path) {
    patch_file(
        openapi_dir.join("apidocs.swagger.json"),
        &Regex::new("version not set").unwrap(),
        OPENAPI_VERSION,
    )
    .expect("error patching apidocs.swagger.json");
}

/// Fix clashing type names in prost-generated code. See cosmos/cosmos-rust#154.
fn apply_proto_patches(proto_dir: &Path) {
    for (pattern, replacement) in [
        ("enum Validators", "enum Policy"),
        (
            "stake_authorization::Validators",
            "stake_authorization::Policy",
        ),
    ] {
        patch_file(
            &proto_dir.join("cosmos.staking.v1beta1.rs"),
            &Regex::new(pattern).unwrap(),
            replacement,
        )
        .expect("error patching cosmos.staking.v1beta1.rs");
    }

    for (pattern, replacement) in [
        (
            "stake_authorization::Validators::Allow",
            "stake_authorization::Policy::Allow",
        ),
        (
            "stake_authorization::Validators::Deny",
            "stake_authorization::Policy::Deny",
        ),
    ] {
        patch_file(
            &proto_dir.join("cosmos.staking.v1beta1.serde.rs"),
            &Regex::new(pattern).unwrap(),
            replacement,
        )
        .expect("error patching cosmos.staking.v1beta1.serde.rs");
    }
}
