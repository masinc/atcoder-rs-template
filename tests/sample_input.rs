use cli_test_dir::*;

const BIN: &str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"erasedream"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str().trim_end_matches('\n'), "YES");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"dreameraser"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str().trim_end_matches('\n'), "YES");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"dreamerer"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str().trim_end_matches('\n'), "NO");
    assert!(output.stderr_str().is_empty());
}
