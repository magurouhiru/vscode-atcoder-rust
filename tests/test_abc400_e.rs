use std::process::Command;

#[test]
fn test_abc400e_0() {
    let input_raw = r#"5
404
36
60
1000000000000
123456789
"#;
    let output_raw = r#"400
36
36
1000000000000
123454321
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc400e"])
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
