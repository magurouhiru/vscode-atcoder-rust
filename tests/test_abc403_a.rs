use std::process::Command;

#[test]
fn test_abc403a_0() {
    let input_raw = r#"7
3 1 4 1 5 9 2
"#;
    let output_raw = r#"14
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc403a"])
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
fn test_abc403a_1() {
    let input_raw = r#"1
100
"#;
    let output_raw = r#"100
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc403a"])
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
fn test_abc403a_2() {
    let input_raw = r#"14
100 10 1 10 100 10 1 10 100 10 1 10 100 10
"#;
    let output_raw = r#"403
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc403a"])
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
