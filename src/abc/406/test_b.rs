use std::process::Command;

#[test]
fn test_abc406b_0() {
    let input_raw = r#"5 2
7 13 3 2 5
"#;
    let output_raw = r#"10
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc406b"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());

    let mut child = cmd.spawn().expect("Failed to spawn process");

    use std::io::Write;
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(input_raw.as_bytes())
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert_eq!(stdout, output_raw);
}

#[test]
fn test_abc406b_1() {
    let input_raw = r#"2 1
2 5
"#;
    let output_raw = r#"1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc406b"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());

    let mut child = cmd.spawn().expect("Failed to spawn process");

    use std::io::Write;
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(input_raw.as_bytes())
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert_eq!(stdout, output_raw);
}
