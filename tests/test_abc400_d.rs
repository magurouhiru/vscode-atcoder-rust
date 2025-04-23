use std::process::Command;

#[test]
fn test_abc400d_0() {
    let input_raw = r#"10 10
..........
#########.
#.......#.
#..####.#.
##....#.#.
#####.#.#.
.##.#.#.#.
###.#.#.#.
###.#.#.#.
#.....#...
1 1 7 1
"#;
    let output_raw = r#"1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc400d"])
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
fn test_abc400d_1() {
    let input_raw = r#"2 2
.#
#.
1 1 2 2
"#;
    let output_raw = r#"1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc400d"])
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
fn test_abc400d_2() {
    let input_raw = r#"1 3
.#.
1 1 1 3
"#;
    let output_raw = r#"1
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc400d"])
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
fn test_abc400d_3() {
    let input_raw = r#"20 20
####################
##...##....###...###
#.....#.....#.....##
#..#..#..#..#..#..##
#..#..#....##..#####
#.....#.....#..#####
#.....#..#..#..#..##
#..#..#.....#.....##
#..#..#....###...###
####################
####################
##..#..##...###...##
##..#..#.....#.....#
##..#..#..#..#..#..#
##..#..#..#..#..#..#
##.....#..#..#..#..#
###....#..#..#..#..#
#####..#.....#.....#
#####..##...###...##
####################
3 3 18 18
"#;
    let output_raw = r#"3
"#;

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "abc400d"])
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
