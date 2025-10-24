use std::process::Command;

#[test]
fn test_abc411e_0() {
    let input_raw = r#"2
1 1 4 4 4 4
1 1 1 3 3 3
"#;
    let output_raw = r#"332748121
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411e"])
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
fn test_abc411e_1() {
    let input_raw = r#"2
1 1 1 1 1 1
2 2 2 2 2 2
"#;
    let output_raw = r#"2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411e"])
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
fn test_abc411e_2() {
    let input_raw = r#"8
55 76 80 21 34 28
82 84 2 32 56 17
11 57 37 28 39 18
47 2 97 25 75 29
72 45 22 75 26 81
6 79 16 68 68 40
31 80 68 57 18 55
49 10 63 91 93 40
"#;
    let output_raw = r#"213725517
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411e"])
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
