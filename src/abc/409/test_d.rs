use std::process::Command;

#[test]
fn test_abc409d_0() {
    let input_raw = r#"3
7
atcoder
1
x
5
snuke
"#;
    let output_raw = r#"acodert
x
nsuke
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc409d"])
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
