extern crate flow;
use tempfile::NamedTempFile;
use std::io::Write;
use flow::rule::{read_flow, read_input, read_str, RulesReader, DecisionReader};

fn create_temp_file(content: &str, extension: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "{}", content).unwrap();
    let path = file.path().to_owned();
    let new_path = path.with_extension(extension);
    std::fs::rename(&path, &new_path).unwrap();
    file
}

#[tokio::test]
async fn test_read_rules_json() {
}

#[tokio::test]
async fn test_read_rules_csv() {
   
}

#[tokio::test]
async fn test_read_flow() {

}

#[tokio::test]
async fn test_read_input() {
}

#[tokio::test]
async fn test_read_str() {
    let content = r#"{"key": "value"}"#;
    let value = read_str(content).await;

    assert_eq!(value["key"], "value");
}

#[tokio::test]
async fn test_error_handling() {
    let result = DecisionReader::read_flow("non_existent_file.json").await;
    assert!(result.is_err());

    let result = DecisionReader::read_input("non_existent_file.json").await;
    assert!(result.is_err());

    let result = DecisionReader::read_str("invalid json").await;
    assert!(result.is_err());
}
