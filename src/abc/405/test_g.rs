use std::process::Command;

#[test]
fn test_abc405g_0() {
    let input_raw = r#"5 3
1 2 3 3 1
1 5 3
3 4 1
1 3 4
"#;
    let output_raw = r#"3
1
6
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405g"])
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
fn test_abc405g_1() {
    let input_raw = r#"8 6
6 2 4 1 5 1 8 6
5 6 3
1 5 7
1 4 6
4 7 8
4 8 2
5 8 6
"#;
    let output_raw = r#"1
120
6
3
1
2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405g"])
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
