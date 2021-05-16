use slash;
use tempfile::NamedTempFile;
use std::io;
use std::io::Read;
use std::cell::RefCell;
use std::path::Path;

pub fn run(src: &str, expected_output: &str) {
    let res = do_run(src).expect("Failed to run");
    assert_eq!(expected_output, &res[..])
}

fn do_run(src: &str) -> io::Result<String> {
    let tmp_stderr = NamedTempFile::new()?;
    let mut tmp_stdout = NamedTempFile::new()?;

    let res = slash::Slash::new(src,
                                Box::new(RefCell::new(tmp_stdout.reopen()?)),
                                Box::new(RefCell::new(tmp_stderr)),
        Path::new("tests/testfiles")
    ).run();
    if let Err(err) = res {
        panic!("{}",err);
    }
    let mut res = String::new();
    tmp_stdout.read_to_string(&mut res).expect("Failed to read result into string");
    Ok(res)
}