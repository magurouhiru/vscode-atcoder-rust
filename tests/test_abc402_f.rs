use std::process::Command;

#[test]
fn test_abc402f_0() {
    let input_raw = r#"2 7
1 2
3 1
"#;
    let output_raw = r#"5
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402f"])
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
fn test_abc402f_1() {
    let input_raw = r#"3 100000
1 2 3
3 5 8
7 1 2
"#;
    let output_raw = r#"13712
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402f"])
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
fn test_abc402f_2() {
    let input_raw = r#"5 402
8 1 3 8 9
8 2 4 1 8
4 1 8 5 9
6 2 1 6 7
6 6 7 7 6
"#;
    let output_raw = r#"384
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402f"])
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
