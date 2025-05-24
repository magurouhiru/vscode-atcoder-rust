use std::process::Command;

#[test]
fn test_abc399b_0() {
    let input_raw = r#"4
3 12 9 9
"#;
    let output_raw = r#"4
1
2
2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc399b"])
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
fn test_abc399b_1() {
    let input_raw = r#"3
3 9 6
"#;
    let output_raw = r#"3
1
2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc399b"])
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
fn test_abc399b_2() {
    let input_raw = r#"4
100 100 100 100
"#;
    let output_raw = r#"1
1
1
1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc399b"])
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
fn test_abc399b_3() {
    let input_raw = r#"8
87 87 87 88 41 38 41 38
"#;
    let output_raw = r#"2
2
2
1
5
7
5
7
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc399b"])
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
