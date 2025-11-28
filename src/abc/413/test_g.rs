use std::process::Command;

#[test]
fn test_abc413g_0() {
    let input_raw = r#"4 5 5
1 4
2 3
3 2
3 4
4 2
"#;
    let output_raw = r#"No
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc413g"])
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
fn test_abc413g_1() {
    let input_raw = r#"2 7 3
1 2
2 4
1 6
"#;
    let output_raw = r#"Yes
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc413g"])
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
fn test_abc413g_2() {
    let input_raw = r#"1 1 0
"#;
    let output_raw = r#"Yes
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc413g"])
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
fn test_abc413g_3() {
    let input_raw = r#"10 12 20
8 3
1 11
6 4
3 7
10 4
5 7
4 7
5 5
4 3
6 1
1 6
2 7
6 7
1 3
6 3
2 12
9 6
7 3
3 11
9 7
"#;
    let output_raw = r#"Yes
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc413g"])
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
