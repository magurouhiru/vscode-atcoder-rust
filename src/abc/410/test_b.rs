use std::process::Command;

#[test]
fn test_abc410b_0() {
    let input_raw = r#"4 5
2 0 3 0 0
"#;
    let output_raw = r#"2 1 3 4 1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc410b"])
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
fn test_abc410b_1() {
    let input_raw = r#"3 7
1 1 0 0 0 0 0
"#;
    let output_raw = r#"1 1 2 3 2 3 1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc410b"])
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
fn test_abc410b_2() {
    let input_raw = r#"6 20
4 6 0 3 4 2 6 5 2 3 0 3 2 5 0 3 5 0 2 0
"#;
    let output_raw = r#"4 6 1 3 4 2 6 5 2 3 1 3 2 5 1 3 5 4 2 6
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc410b"])
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
