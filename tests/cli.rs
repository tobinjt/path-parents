use std::process::Command;

#[test]
fn test_cli_parents() {
    let bin = env!("CARGO_BIN_EXE_path-parents");
    let output = Command::new(bin)
        .arg("/usr/bin/cat")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "/usr\n/usr/bin\n/usr/bin/cat");
}

#[test]
fn test_cli_skip() {
    let bin = env!("CARGO_BIN_EXE_path-parents");
    let output = Command::new(bin)
        .args(["--skip", "1", "/usr/bin/cat"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "/usr/bin\n/usr/bin/cat");
}

#[test]
fn test_cli_stdin() {
    use std::io::Write;
    let bin = env!("CARGO_BIN_EXE_path-parents");
    let mut child = Command::new(bin)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(b"/tmp/foo/bar\n")
            .expect("failed to write to stdin");
    });

    let output = child.wait_with_output().expect("failed to wait on child");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "/tmp\n/tmp/foo\n/tmp/foo/bar");
}

#[test]
fn test_cli_error() {
    let bin = env!("CARGO_BIN_EXE_path-parents");
    let output = Command::new(bin)
        .arg("--invalid-flag")
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());
    // Clap handles invalid flags and exits before our main's match.
}

#[test]
fn test_cli_stdin_invalid_utf8() {
    use std::io::Write;
    let bin = env!("CARGO_BIN_EXE_path-parents");
    let mut child = Command::new(bin)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    std::thread::spawn(move || {
        // Invalid UTF-8 sequence
        let _ = stdin.write_all(b"/foo/\xffbar\n");
    });

    let output = child.wait_with_output().expect("failed to wait on child");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:"));
}
