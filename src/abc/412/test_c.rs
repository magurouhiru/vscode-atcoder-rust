use std::process::Command;

#[test]
fn test_abc412c_0() {
    let input_raw = r#"3
4
1 3 2 5
2
1 100
10
298077099 766294630 440423914 59187620 725560241 585990757 965580536 623321126 550925214 917827435
"#;
    let output_raw = r#"4
-1
3
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc412c"])
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
