use std::process::Command;

#[test]
fn test_abc400g_0() {
    let input_raw = r#"1
3 1
6 3 8
3 5 0
2 7 3
"#;
    let output_raw = r#"12
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc400g"])
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
fn test_abc400g_1() {
    let input_raw = r#"2
5 2
1 2 3
1 2 3
1 2 3
1 2 3
100 100 200
6 2
21 74 25
44 71 80
46 28 96
1 74 24
81 83 16
55 31 1
"#;
    let output_raw = r#"209
333
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc400g"])
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
