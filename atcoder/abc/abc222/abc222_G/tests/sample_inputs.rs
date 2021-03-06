use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1
7
10
999983
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
6
-1
999982
"#);
    assert!(output.stderr_str().is_empty());
}
