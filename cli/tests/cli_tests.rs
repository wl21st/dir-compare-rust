use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};

fn cli_command() -> Command {
    cargo_bin_cmd!("dir-compare")
}

fn setup_test_dirs() -> tempfile::TempDir {
    let temp_dir = tempfile::tempdir().unwrap();

    let dir_a = temp_dir.path().join("dir_a");
    let dir_b = temp_dir.path().join("dir_b");
    fs::create_dir_all(&dir_a).unwrap();
    fs::create_dir_all(&dir_b).unwrap();

    fs::write(dir_a.join("file1.txt"), b"content1").unwrap();
    fs::write(dir_b.join("file1.txt"), b"content1").unwrap();

    temp_dir
}

#[test]
fn test_cli_missing_dir_a() {
    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg("/nonexistent")
        .arg("--dir-b")
        .arg("/tmp")
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_cli_missing_dir_b() {
    let temp_dir = tempfile::tempdir().unwrap();
    let dir_a = temp_dir.path().join("dir_a");
    fs::create_dir_all(&dir_a).unwrap();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(dir_a.to_str().unwrap())
        .arg("--dir-b")
        .arg("/nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_cli_path_is_not_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_a = temp_dir.path().join("file_a");
    File::create(&file_a).unwrap();
    let dir_b = temp_dir.path().join("dir_b");
    fs::create_dir_all(&dir_b).unwrap();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(file_a.to_str().unwrap())
        .arg("--dir-b")
        .arg(dir_b.to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a directory"));
}

#[test]
fn test_cli_invalid_method() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("--method")
        .arg("invalid_method")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid comparison method"));
}

#[test]
fn test_cli_invalid_format() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("--format")
        .arg("invalid_format")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid format"));
}

#[test]
fn test_cli_default_method() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Both"));
}

#[test]
fn test_cli_method_filename() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("--method")
        .arg("filename")
        .assert()
        .success();
}

#[test]
fn test_cli_method_size() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("--method")
        .arg("size")
        .assert()
        .success();
}

#[test]
fn test_cli_method_hash() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("--method")
        .arg("hash")
        .assert()
        .success();
}

#[test]
fn test_cli_case_insensitive() {
    let temp_dir = tempfile::tempdir().unwrap();

    let dir_a = temp_dir.path().join("dir_a");
    let dir_b = temp_dir.path().join("dir_b");
    fs::create_dir_all(&dir_a).unwrap();
    fs::create_dir_all(&dir_b).unwrap();

    fs::write(dir_a.join("File.Txt"), b"content").unwrap();
    fs::write(dir_b.join("file.txt"), b"content").unwrap();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(dir_a.to_str().unwrap())
        .arg("--dir-b")
        .arg(dir_b.to_str().unwrap())
        .arg("--case-insensitive")
        .assert()
        .success()
        .stdout(predicate::str::contains("Both"));
}

#[test]
fn test_cli_format_text() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("--format")
        .arg("text")
        .assert()
        .success()
        .stdout(predicate::str::contains("A-only"));
}

#[test]
fn test_cli_format_html() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("--format")
        .arg("html")
        .assert()
        .success()
        .stdout(predicate::str::contains("<!DOCTYPE html>"));
}

#[test]
fn test_cli_format_markdown() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("--format")
        .arg("markdown")
        .assert()
        .success()
        .stdout(predicate::str::contains("# Directory Comparison Report"));
}

#[test]
fn test_cli_output_file() {
    let temp_dir = setup_test_dirs();
    let output_file = temp_dir.path().join("output.txt");

    let mut cmd = cli_command();
    cmd.arg("--dir-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("--dir-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("--output")
        .arg(output_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Report written to"));

    assert!(output_file.exists());
    let content = fs::read_to_string(output_file).unwrap();
    assert!(content.contains("Both"));
}

#[test]
fn test_cli_short_flags() {
    let temp_dir = setup_test_dirs();

    let mut cmd = cli_command();
    cmd.arg("-a")
        .arg(temp_dir.path().join("dir_a").to_str().unwrap())
        .arg("-b")
        .arg(temp_dir.path().join("dir_b").to_str().unwrap())
        .arg("-m")
        .arg("filename")
        .arg("-f")
        .arg("text")
        .assert()
        .success();
}

#[test]
fn test_cli_help_message() {
    let mut cmd = cli_command();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("dir-compare"))
        .stdout(predicate::str::contains("--dir-a"))
        .stdout(predicate::str::contains("--dir-b"))
        .stdout(predicate::str::contains("--method"))
        .stdout(predicate::str::contains("--format"));
}

#[test]
fn test_cli_version() {
    let mut cmd = cli_command();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}
