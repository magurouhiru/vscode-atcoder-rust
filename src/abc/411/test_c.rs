use std::process::Command;

#[test]
fn test_abc411c_0() {
    let input_raw = r#"5 7
2 3 3 5 1 5 2
"#;
    let output_raw = r#"1
1
1
2
2
1
1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411c"])
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
fn test_abc411c_1() {
    let input_raw = r#"1 2
1 1
"#;
    let output_raw = r#"1
0
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411c"])
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
fn test_abc411c_2() {
    let input_raw = r#"3 3
1 3 2
"#;
    let output_raw = r#"1
2
1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411c"])
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
