use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1 2 2
2 3 3
1 2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"8
6
6
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
1 2 3
1 3 1
1 4 4
1 5 1
1 6 5
9 2 6 5 3 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"105
108
106
109
106
14
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
1 2 1000000000
2 3 1000000000
3 4 1000000000
4 5 1000000000
5 6 1000000000
1 2 3 4 5 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"5000000006
4000000006
3000000006
3000000001
4000000001
5000000001
"#);
    assert!(output.stderr_str().is_empty());
}
