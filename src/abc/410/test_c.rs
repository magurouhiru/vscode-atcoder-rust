use std::process::Command;

#[test]
fn test_abc410c_0() {
    let input_raw = r#"5 5
2 3
1 2 1000000
3 4
2 2
2 3
"#;
    let output_raw = r#"3
1
1000000
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc410c"])
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
fn test_abc410c_1() {
    let input_raw = r#"1000000 2
1 1000000 999999
3 1000000000
"#;
    let output_raw = r#""#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc410c"])
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
