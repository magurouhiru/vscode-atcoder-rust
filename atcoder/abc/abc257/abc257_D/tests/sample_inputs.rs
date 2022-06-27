use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
1 1
2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"5
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
2 2 2
2 2 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
1 2 3 4 5 6 7 8 9 10
1 4 9 16 25 36 49 64 81 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"978222082
"#);
    assert!(output.stderr_str().is_empty());
}
