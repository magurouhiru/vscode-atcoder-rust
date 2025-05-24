use std::process::Command;

#[test]
fn test_abc402c_0() {
    let input_raw = r#"5 4
2 1 2
3 3 4 5
3 1 2 5
1 3
1 3 2 5 4
"#;
    let output_raw = r#"0
1
2
3
4
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402c"])
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
fn test_abc402c_1() {
    let input_raw = r#"9 8
1 4
5 6 9 7 4 3
4 2 4 1 3
1 1
5 7 9 8 1 5
2 9 8
1 2
1 1
6 5 2 7 8 4 1 9 3
"#;
    let output_raw = r#"0
0
1
1
1
2
4
6
8
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402c"])
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
