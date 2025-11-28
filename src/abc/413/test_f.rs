use std::process::Command;

#[test]
fn test_abc413f_0() {
    let input_raw = r#"2 3 2
1 2
2 1
"#;
    let output_raw = r#"2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc413f"])
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
fn test_abc413f_1() {
    let input_raw = r#"9 3 9
1 3
6 1
4 1
1 2
2 1
7 1
9 3
8 1
9 2
"#;
    let output_raw = r#"43
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc413f"])
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
fn test_abc413f_2() {
    let input_raw = r#"10 10 36
3 8
5 10
3 10
6 10
2 10
2 8
7 10
1 10
1 8
7 6
7 8
2 5
1 6
8 8
7 5
2 4
9 8
7 4
4 3
10 10
10 8
8 10
10 6
6 2
4 2
10 5
8 3
1 2
2 1
4 1
10 4
10 3
8 1
6 1
10 2
9 1
"#;
    let output_raw = r#"153
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc413f"])
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
