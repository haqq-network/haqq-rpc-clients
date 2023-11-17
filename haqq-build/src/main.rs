use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fs, io};

use regex::Regex;

const PROTO_GEN_DIR: &str = "src/gen";
const PROTO_WEB_DIR: &str = "web/gen";
const PROTO_TMP_DIR: &str = "tmp-proto";
// const OPENAPI_SPEC_GEN_DIR: &str = "openapi";
// const OPENAPI_VERSION: &str = "0.1.0";
// const OPENAPI_GEN_DIR: &str = "rest";

fn main() {
    if std::path::Path::new(PROTO_GEN_DIR).exists() {
        fs::remove_dir_all(PROTO_GEN_DIR).unwrap();
        fs::remove_dir_all(PROTO_WEB_DIR).unwrap();
        fs::remove_dir_all(PROTO_TMP_DIR).ok();
    }

    run_command("mkdir", ["-p", "tmp-proto/proto"]);

    let yq = r#"cat haqq-node/proto/buf.yaml | yq '.deps | map( "buf export " + . + " -o 'tmp-proto/proto'") | join(" && ")' | xargs bash -c"#;
    run_command("bash", ["-c", yq]);
    run_command(
        "bash",
        ["-c", "buf export haqq-node/proto -o tmp-proto/proto"],
    );

    // run_command("bash", ["-c", "cp buf.yaml tmp-proto"]);
    // run_command("bash", ["-c", "cp buf.lock tmp-proto"]);

    run_command(
        "buf",
        [
            "generate",
            "--exclude-path",
            "tmp-proto/proto/google",
            "--exclude-path",
            "tmp-proto/proto/gogoproto",
            "--exclude-path",
            "tmp-proto/proto/cosmos/nft",
        ],
    );

    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/cosmos.msg.v1.rs", PROTO_GEN_DIR))
        .unwrap();

    f.write_all(b"// an empty stub, needed because prost-crate expects it to be there while prost doesn't generate the file").unwrap();
    std::mem::drop(f);

    apply_proto_patches(Path::new(PROTO_GEN_DIR));

    // if std::path::Path::new(OPENAPI_GEN_DIR).exists() {
    //     fs::remove_dir_all(OPENAPI_GEN_DIR).unwrap();
    // }

    // let mut cmd = Command::new("openapi-generator-cli");
    // cmd.args([
    //     "generate",
    //     "-i",
    //     "haqq-node/client/docs/swagger-ui/swagger.json",
    //     "-g",
    //     "rust",
    //     // "--enum-name-mappings",
    //     // "Option=Opt,option=opt",
    //     "-o",
    //     OPENAPI_GEN_DIR,
    //     "--skip-validate-spec",
    //     "--package-name",
    //     "haqq-rest",
    // ]);

    // cmd.stdout(stdout).stderr(stderr);

    // let output = cmd.output().unwrap();
    // assert!(output.status.success());

    // apply_rest_patches();
}

// https://github.com/cosmos/cosmos-rust/blob/08353cb090aea4ad1569c5962a1a92d6327cd2d1/proto-build/src/main.rs#L396
fn patch_file(path: impl AsRef<Path>, pattern: &Regex, replacement: &str) -> io::Result<()> {
    let mut contents = fs::read_to_string(&path)?;
    contents = pattern.replace_all(&contents, replacement).to_string();
    fs::write(&path, &contents)
}

// fn patch_files(paths: Vec<impl AsRef<Path>>, pattern: &Regex, replacement: &str) -> io::Result<()> {
//     for path in paths {
//         if let Err(e) = patch_file(path, pattern, replacement) {
//             return Err(e);
//         }
//     }

//     Ok(())
// }

// fn apply_rest_patches() {
//     for (pattern, replacement) in [
//         ("Option<", "core::option::Option<"),
//         ("Option::is_", "core::option::Option::is_"),
//     ] {
//         // FIXME: figure out a way how to rename Option in enum
//         // there is --enum-name-mappings option in openapi-generator-cli
//         // but apparently it's currently only supported by java generators
//         // patch_files(
//         //     vec![
//         //         "rest/src/models/cosmos_period_gov_period_v1_period_weighted_vote_option.rs",
//         //         "rest/src/models/cosmos_period_gov_period_v1beta1_period_vote.rs",
//         //         "rest/src/models/cosmos_period_gov_period_v1beta1_period_weighted_vote_option.rs",
//         //         "rest/src/models/gov_v1_votes_200_response_votes_inner_options_inner.rs",
//         //         "rest/src/models/votes_200_response_votes_inner_options_inner.rs",
//         //         "rest/src/models/votes_200_response_votes_inner.rs",
//         //         "rest/src/models/vote_200_response_vote.rs",
//         //     ],
//         //     &Regex::new(pattern).unwrap(),
//         //     replacement,
//         // )
//         // .expect("error patching rest file");
//     }
// }

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

fn run_command<I, S>(cmd: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let stdout = Stdio::inherit();
    let stderr = Stdio::inherit();

    let mut cmd = Command::new(cmd);
    cmd.args(args);

    cmd.stdout(stdout).stderr(stderr);

    let output = cmd.output().unwrap();
    assert!(output.status.success());
}
