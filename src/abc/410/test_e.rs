use std::process::Command;

#[test]
fn test_abc410e_0() {
    let input_raw = r#"4 10 14
5 8
5 6
7 9
99 99
"#;
    let output_raw = r#"3
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc410e"])
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
fn test_abc410e_1() {
    let input_raw = r#"3 3000 3000
3 3
3 3
3 3
"#;
    let output_raw = r#"3
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc410e"])
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
fn test_abc410e_2() {
    let input_raw = r#"10 8 8
2 2
2 3
2 2
1 2
2 3
1 2
3 3
3 2
3 1
3 2
"#;
    let output_raw = r#"9
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc410e"])
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
