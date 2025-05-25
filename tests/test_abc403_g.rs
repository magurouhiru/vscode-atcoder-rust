use std::process::Command;

#[test]
fn test_abc403g_0() {
    let input_raw = r#"5
1
3
1
999999994
999999993
"#;
    let output_raw = r#"2
2
8
6
1000000006
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc403g"])
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
fn test_abc403g_1() {
    let input_raw = r#"8
105282053
695234822
468007124
120710491
568831200
700753895
765188109
262666319
"#;
    let output_raw = r#"105282054
105282054
905798931
599798602
995656103
891549225
1652393438
1652393438
105282054
800516877
573289179
26509423
168629803
696409999
656737335
915059758
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc403g"])
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
