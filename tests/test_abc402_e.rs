use std::process::Command;

#[test]
fn test_abc402e_0() {
    let input_raw = r#"3 2
100 1 50
200 1 20
1000 1 1
"#;
    let output_raw = r#"95
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402e"])
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
fn test_abc402e_1() {
    let input_raw = r#"2 7
100 3 50
100 2 50
"#;
    let output_raw = r#"125
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402e"])
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
fn test_abc402e_2() {
    let input_raw = r#"5 32
500 9 57
300 4 8
300 3 32
300 7 99
100 8 69
"#;
    let output_raw = r#"953.976967020096
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402e"])
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
fn test_abc402e_3() {
    let input_raw = r#"7 78
100 1 100
200 2 90
300 3 80
400 4 60
450 5 50
525 6 30
650 7 1
"#;
    let output_raw = r#"1976.2441416041121021
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402e"])
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
