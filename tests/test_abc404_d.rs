use std::process::Command;

#[test]
fn test_abc404d_0() {
    let input_raw = r#"4 3
1000 300 700 200
3 1 3 4
3 1 2 4
2 1 3
"#;
    let output_raw = r#"1800
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404d"])
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
fn test_abc404d_1() {
    let input_raw = r#"7 6
500 500 500 500 500 500 1000
3 1 2 7
3 2 3 7
3 3 4 7
3 4 5 7
3 5 6 7
3 6 1 7
"#;
    let output_raw = r#"2000
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404d"])
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
