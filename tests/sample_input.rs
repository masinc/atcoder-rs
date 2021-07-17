use cli_test_dir::*;

const BIN: &str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3
8 12 40
"#,
        )
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
        .output_with_stdin(
            r#"4
5 6 8 10
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str().trim_end_matches('\n'), "0");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6
382253568 723152896 37802240 379425024 404894720 471526144
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str().trim_end_matches('\n'), "8");
    assert!(output.stderr_str().is_empty());
}
