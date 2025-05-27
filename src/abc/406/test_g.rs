use std::process::Command;

#[test]
fn test_abc406g_0() {
    let input_raw = r#"3 2 3
1 -1 2
"#;
    let output_raw = r#"10
0 0 2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc406g"])
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
fn test_abc406g_1() {
    let input_raw = r#"2 100000 60000
100000 -100000
"#;
    let output_raw = r#"12000000000
0 0
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc406g"])
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
fn test_abc406g_2() {
    let input_raw = r#"6 4 4
2 -1 5 -2 -2 2
"#;
    let output_raw = r#"56
0 -1 -1 -2 -2 2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc406g"])
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
