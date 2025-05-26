use std::process::Command;

#[test]
fn test_abc405f_0() {
    let input_raw = r#"4 2
2 4
6 8
3
1 3
3 7
1 5
"#;
    let output_raw = r#"1
2
0
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405f"])
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
fn test_abc405f_1() {
    let input_raw = r#"20 7
24 34
26 28
18 38
2 14
8 12
30 32
20 22
10
7 29
31 39
9 21
19 29
15 21
11 39
17 21
15 31
5 25
25 31
"#;
    let output_raw = r#"3
3
4
1
2
2
2
3
3
1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405f"])
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
