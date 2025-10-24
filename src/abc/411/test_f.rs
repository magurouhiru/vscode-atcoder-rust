use std::process::Command;

#[test]
fn test_abc411f_0() {
    let input_raw = r#"7 7
1 2
1 3
2 3
1 4
1 5
2 5
6 7
5
1 2 3 1 5
"#;
    let output_raw = r#"4
3
3
3
2
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc411f"])
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
