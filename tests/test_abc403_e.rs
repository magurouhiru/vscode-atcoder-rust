use std::process::Command;

#[test]
fn test_abc403e_0() {
    let input_raw = r#"4
1 at
2 watcoder
2 atcoder
1 wa
"#;
    let output_raw = r#"0
1
1
0
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc403e"])
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
fn test_abc403e_1() {
    let input_raw = r#"10
1 w
1 avko
2 atcoder
1 bzginn
2 beginner
1 atco
2 contest
1 ntxcdg
1 atc
1 contest
"#;
    let output_raw = r#"0
0
1
1
2
1
2
2
2
1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc403e"])
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
