use cli_test_dir::*;

const BIN: &str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"101"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str().trim_end_matches('\n'), "2");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"000"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str().trim_end_matches('\n'), "0");
    assert!(output.stderr_str().is_empty());
}
