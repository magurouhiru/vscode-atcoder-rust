use std::process::Command;

#[test]
fn test_abc405c_0() {
    let input_raw = r#"3
4 2 3
"#;
    let output_raw = r#"26
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405c"])
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
fn test_abc405c_1() {
    let input_raw = r#"2
9 45
"#;
    let output_raw = r#"405
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405c"])
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
fn test_abc405c_2() {
    let input_raw = r#"10
7781 8803 8630 9065 8831 9182 8593 7660 7548 8617
"#;
    let output_raw = r#"3227530139
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405c"])
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
