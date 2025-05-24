use std::process::Command;

#[test]
fn test_abc402d_0() {
    let input_raw = r#"8 3
1 5
1 8
2 4
"#;
    let output_raw = r#"2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402d"])
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
fn test_abc402d_1() {
    let input_raw = r#"5 10
2 5
1 5
1 2
2 4
2 3
1 3
1 4
3 5
3 4
4 5
"#;
    let output_raw = r#"40
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402d"])
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
