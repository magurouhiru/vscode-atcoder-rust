use std::process::Command;

#[test]
fn test_abc404c_0() {
    let input_raw = r#"4 4
2 4
3 1
4 1
2 3
"#;
    let output_raw = r#"Yes
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404c"])
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
fn test_abc404c_1() {
    let input_raw = r#"4 6
1 2
1 3
1 4
2 3
2 4
3 4
"#;
    let output_raw = r#"No
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404c"])
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
