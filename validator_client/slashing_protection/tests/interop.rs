use slashing_protection::interchange_test::MultiTestCase;
use std::fs::File;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static TEST_ROOT_DIR: LazyLock<PathBuf> = LazyLock::new(test_root_dir);

fn download_tests() {
    let make_output = std::process::Command::new("make")
        .current_dir(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .output()
        .expect("need `make` to succeed to download and untar slashing protection tests");
    if !make_output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&make_output.stderr));
        panic!("Running `make` for slashing protection tests failed, see above");
    }
}

fn test_root_dir() -> PathBuf {
    download_tests();
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("interchange-tests")
        .join("tests")
}

// NOTE: I've combined two tests together to avoid a race-condition which occurs when fighting over
// which test builds the TEST_ROOT_DIR lazy static.
#[test]
fn generated_and_with_minification() {
    for entry in TEST_ROOT_DIR
        .join("generated")
        .read_dir()
        .unwrap()
        .map(Result::unwrap)
    {
        let file = File::open(entry.path()).unwrap();
        let test_case: MultiTestCase = serde_json::from_reader(&file).unwrap();
        test_case.run(false);
    }

    for entry in TEST_ROOT_DIR
        .join("generated")
        .read_dir()
        .unwrap()
        .map(Result::unwrap)
    {
        let file = File::open(entry.path()).unwrap();
        let test_case: MultiTestCase = serde_json::from_reader(&file).unwrap();
        test_case.run(true);
    }
}
