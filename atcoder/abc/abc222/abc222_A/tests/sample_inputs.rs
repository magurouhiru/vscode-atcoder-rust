use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"321
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0321
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7777
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"7777
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0001
"#);
    assert!(output.stderr_str().is_empty());
}
