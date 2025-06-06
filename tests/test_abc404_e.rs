use std::process::Command;

#[test]
fn test_abc404e_0() {
    let input_raw = r#"5
1 1 2 1
1 0 0 1
"#;
    let output_raw = r#"3
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404e"])
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
fn test_abc404e_1() {
    let input_raw = r#"6
1 2 1 3 1
1 1 0 1 1
"#;
    let output_raw = r#"4
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404e"])
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
fn test_abc404e_2() {
    let input_raw = r#"16
1 1 1 2 5 1 1 3 4 1 4 3 1 1 2
1 0 0 0 1 0 0 1 1 0 0 0 0 0 1
"#;
    let output_raw = r#"7
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404e"])
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
