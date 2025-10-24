use std::process::Command;

#[test]
fn test_abc411d_0() {
    let input_raw = r#"2 6
2 1 at
3 1
2 2 on
1 2
2 2 coder
3 2
"#;
    let output_raw = r#"atcoder
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411d"])
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
fn test_abc411d_1() {
    let input_raw = r#"100000 3
1 100
2 300 abc
3 200
"#;
    let output_raw = r#""#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411d"])
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
fn test_abc411d_2() {
    let input_raw = r#"10 10
2 7 ladxf
2 7 zz
2 7 kfm
3 7
1 5
2 5 irur
3 5
1 6
2 6 ptilun
3 6
"#;
    let output_raw = r#"ladxfzzkfmirurptilun
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411d"])
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
