use slash;
use tempfile::NamedTempFile;
use std::io;
use std::io::Read;
use std::cell::RefCell;
use std::path::PathBuf;

pub fn run(src: &str, expected_output: &str) {
    let (stdout, stderr) = do_run(src).expect("Failed to run");
    if !"".eq(&stderr) {
        panic!("Expected no data on stderr, but got:\n{}",stderr);
    } else {
        assert_eq!(expected_output, &stdout)
    }
}

fn do_run(src: &str) -> io::Result<(String, String)> {
    let mut tmp_stderr = NamedTempFile::new()?;
    let mut tmp_stdout = NamedTempFile::new()?;

    let res = slash::Slash::new(src,
                                Box::new(RefCell::new(tmp_stdout.reopen()?)),
                                Box::new(RefCell::new(tmp_stderr.reopen()?)),
                                PathBuf::from("tests/testfiles"),
                                vec!()
    ).run();
    if let Err(err) = res {
        panic!("{}",err);
    }
    let mut stdout_res = String::new();
    tmp_stdout.read_to_string(&mut stdout_res).expect("Failed to read stdout result into string");
    let mut stderr_res = String::new();
    tmp_stderr.read_to_string(&mut stderr_res).expect("Failed to read stderr result into string");
    Ok((stdout_res, stderr_res))
}