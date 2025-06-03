use std::process::Command;

#[test]
fn test_version() {
    let exe = env!("CARGO_BIN_EXE_commitfmt");

    let output = Command::new(exe).arg("--version").output().unwrap();
    assert!(output.status.success());
    let version = String::from_utf8(output.stdout).unwrap();

    let expected = format!("commitfmt {}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(version, expected);
}
