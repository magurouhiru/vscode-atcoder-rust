use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 5 0
2 3 2 1 4
1 2
2 3
3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 10 10000
1 2 1 2 1 2 2 1 1 2
1 2
1 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 2 -1
1 10
1 2
2 3
3 4
4 5
5 6
6 7
7 8
8 9
9 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"126
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 8 -1
1 4 1 4 2 1 3 5
1 2
4 1
3 1
1 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
"#);
    assert!(output.stderr_str().is_empty());
}
