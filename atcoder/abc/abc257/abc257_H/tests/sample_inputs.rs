use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"6
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"222
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"987355927
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"222222
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"675337738
"#);
    assert!(output.stderr_str().is_empty());
}
