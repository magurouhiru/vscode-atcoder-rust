use std::process::Command;

#[test]
fn test_abc405b_0() {
    let input_raw = r#"5 3
3 2 3 1 2
"#;
    let output_raw = r#"2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405b"])
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
fn test_abc405b_1() {
    let input_raw = r#"4 3
1 3 1 3
"#;
    let output_raw = r#"0
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405b"])
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
fn test_abc405b_2() {
    let input_raw = r#"10 4
1 3 3 4 2 1 3 1 2 4
"#;
    let output_raw = r#"6
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405b"])
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
