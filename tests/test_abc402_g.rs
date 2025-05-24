use std::process::Command;

#[test]
fn test_abc402g_0() {
    let input_raw = r#"5
4 7 2 1 4
12 15 2 8 7
777 1 0 0 0
100 101 0 100 100
402 402 4 19 256
"#;
    let output_raw = r#"27
866
0
1000000
13728568
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc402g"])
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
