use std::process::Command;

#[test]
fn test_abc413e_0() {
    let input_raw = r#"4
1
1 2
2
1 3 4 2
2
2 3 4 1
3
8 3 4 2 1 5 7 6
"#;
    let output_raw = r#"1 2
1 3 2 4
1 4 2 3
1 5 6 7 2 4 3 8
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc413e"])
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
