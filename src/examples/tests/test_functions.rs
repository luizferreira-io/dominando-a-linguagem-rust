pub fn test_success(file_path: &str, stdout: &'static str) {
    assert_cmd::Command::new("cargo")
        .arg("-q")
        .arg("-Zscript")
        .arg(file_path)
        .arg("run")
        .assert()
        .success()
        .stdout(stdout);
}

pub fn test_failure(file_path: &str) {
    assert_cmd::Command::new("cargo")
        .arg("-q")
        .arg("-Zscript")
        .arg(file_path)
        .arg("run")
        .assert()
        .failure();
}
