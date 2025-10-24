use std::process::Command;

#[test]
fn test_abc409e_0() {
    let input_raw = r#"4
-3 2 2 -1
1 2 2
1 3 1
1 4 3
"#;
    let output_raw = r#"9
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc409e"])
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
fn test_abc409e_1() {
    let input_raw = r#"2
0 0
1 2 1
"#;
    let output_raw = r#"0
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc409e"])
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
fn test_abc409e_2() {
    let input_raw = r#"5
-2 -8 10 -2 2
3 5 1
1 3 5
2 5 0
3 4 6
"#;
    let output_raw = r#"28
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc409e"])
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
