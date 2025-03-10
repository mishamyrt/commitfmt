#![cfg(test)]
extern crate test_generator;

use std::path::PathBuf;

use commitfmt_hook::manager::detect_from_path;
use test_generator::test_resources;

#[test_resources("resources/test/*.sh")]
fn test_detect_from_path(resource: &str) {
    let path = PathBuf::from(resource);
    let expected_name = path.file_stem().unwrap().to_str().unwrap();

    let result = detect_from_path(&path).unwrap();

    if expected_name == "unknown" {
        assert!(result.is_none());
    } else {
        let manager = result.unwrap();
        assert_eq!(manager.name, expected_name);
    }
}
