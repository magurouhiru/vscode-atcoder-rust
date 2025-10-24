use std::process::Command;

#[test]
fn test_abc411b_0() {
    let input_raw = r#"5
5 10 2 3
"#;
    let output_raw = r#"5 15 17 20
10 12 15
2 5
3
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411b"])
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
fn test_abc411b_1() {
    let input_raw = r#"2
100
"#;
    let output_raw = r#"100
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411b"])
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
