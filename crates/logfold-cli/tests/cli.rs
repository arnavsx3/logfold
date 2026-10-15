use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_logfold"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start logfold");
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn folds_stdin_with_stable_ordering() {
    let output = run(&[], b"user 1 in\nuser 2 in\nzebra ready\nalpha ready\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "     2x  user <NUM> in\n     1x  alpha ready\n     1x  zebra ready\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn accepts_explicit_stdin_and_windows_line_endings() {
    let output = run(&["-"], b"user 1 in\r\nuser 2 in");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"     2x  user <NUM> in\n");
}

#[test]
fn reads_sample_file() {
    let sample = concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/app.log");
    let output = run(&[sample], b"");
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"     3x  user <NUM> logged in from <IP>\n     2x  timeout after <NUM>s on db-<NUM>\n     1x  request <UUID> finished in <NUM>ms\n"
    );
}

#[test]
fn limits_output() {
    let output = run(&["--top", "1"], b"user 1 in\nuser 2 in\nserver started\n");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"     2x  user <NUM> in\n");
    assert!(run(&["-t", "0"], b"server started\n").stdout.is_empty());
}

#[test]
fn empty_input_produces_no_output() {
    let output = run(&[], b"");
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn reports_missing_files() {
    let output = run(&["/logfold-nonexistent-directory/missing.log"], b"");
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("cannot open")
    );
}

#[test]
fn reports_invalid_utf8_with_line_number() {
    let output = run(&[], b"server started\n\xff\n");
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("cannot read stdin at line 2")
    );
}

#[test]
fn rejects_invalid_limits() {
    for top in ["nope", "-1"] {
        let output = run(&["--top", top], b"");
        assert!(!output.status.success());
        assert!(!output.stderr.is_empty());
    }
}

#[test]
fn help_describes_the_command() {
    let output = run(&["--help"], b"");
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("Usage: logfold"));
    assert!(help.contains("--top"));
    assert!(help.contains("stdin"));
}

#[test]
fn closed_output_pipe_exits_cleanly() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_logfold"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"server started\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}
