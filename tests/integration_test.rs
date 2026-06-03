use std::path::PathBuf;
use std::process::Command;

/// Helper to get the project binary path
fn binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("smart-shell-complete");
    path
}

#[test]
fn test_cli_help() {
    let output = Command::new(binary_path())
        .arg("--help")
        .output()
        .expect("Failed to run smart-shell-complete --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("smart-shell-complete"));
    assert!(stdout.contains("learn"));
    assert!(stdout.contains("predict"));
    assert!(stdout.contains("complete"));
    assert!(stdout.contains("stats"));
    assert!(stdout.contains("install"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(binary_path())
        .arg("--version")
        .output()
        .expect("Failed to run smart-shell-complete --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty());
}

#[test]
fn test_cli_stats_empty_db() {
    // Run stats on a non-existent db (should create it)
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let db_path = dir.path().join("test.db");

    let output = Command::new(binary_path())
        .arg("--db")
        .arg(&db_path)
        .arg("stats")
        .output()
        .expect("Failed to run stats");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Unique commands"));
    assert!(stdout.contains("0"));
}

#[test]
fn test_cli_predict_empty_db() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let db_path = dir.path().join("test.db");

    let output = Command::new(binary_path())
        .arg("--db")
        .arg(&db_path)
        .arg("predict")
        .output()
        .expect("Failed to run predict");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No predictions available"));
}

#[test]
fn test_cli_complete_empty_db() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let db_path = dir.path().join("test.db");

    let output = Command::new(binary_path())
        .arg("--db")
        .arg(&db_path)
        .arg("complete")
        .arg("--prefix")
        .arg("git")
        .output()
        .expect("Failed to run complete");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No completions found"));
}

#[test]
fn test_cli_install_unsupported_shell() {
    let output = Command::new(binary_path())
        .arg("install")
        .arg("unsupported")
        .output()
        .expect("Failed to run install");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unsupported shell"));
}
