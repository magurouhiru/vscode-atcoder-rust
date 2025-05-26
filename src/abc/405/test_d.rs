use std::process::Command;

#[test]
fn test_abc405d_0() {
    let input_raw = r#"3 4
...E
.#..
....
"#;
    let output_raw = r#">>>E
^#>^
>>>^
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405d"])
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
fn test_abc405d_1() {
    let input_raw = r#"3 2
##
##
##
"#;
    let output_raw = r#"##
##
##
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405d"])
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
fn test_abc405d_2() {
    let input_raw = r#"7 20
....................
..#..#..####..#E##..
..#..#..#..#..#.....
..E###..#..#..####..
.....#..#..E.....#..
.....#..####..####..
....................
"#;
    let output_raw = r#">v<<<<<>>>>>>>>v<<<<
>v#^<#^^####v^#E##vv
>v#^<#v^#>v#vv#^<<<<
>>E###vv#>v#vv####^<
>>^<<#vv#>>E<<<<<#^<
>>^<<#vv####^<####^<
>>^<<<<<>>>>^<<<<<^<
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc405d"])
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
