use logging_extractor::{extract_dir, LoggingStatement};
use test_case::test_case;

#[test_case("DefaultShareProvider")]
fn snapshot_test(name: &str) {
    let root = format!("test-data/{name}");
    let mut output = Vec::<u8>::with_capacity(1024 * 1024);
    extract_dir(&root, &mut output).unwrap();
    let output: Vec<LoggingStatement> = serde_json::from_slice(&output).unwrap();

    insta::assert_json_snapshot!(output)
}
