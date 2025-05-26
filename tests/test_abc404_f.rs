use std::process::Command;

#[test]
fn test_abc404f_0() {
    let input_raw = r#"3 2 2 3
"#;
    let output_raw = r#"0.222222222222222
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404f"])
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
fn test_abc404f_1() {
    let input_raw = r#"10 1 10 1
"#;
    let output_raw = r#"1.000000000000000
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404f"])
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
fn test_abc404f_2() {
    let input_raw = r#"100 10 10 2
"#;
    let output_raw = r#"0.401263060761621
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc404f"])
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
