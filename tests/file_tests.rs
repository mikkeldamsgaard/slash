use std::fs;

mod common;

#[test]
fn test() {
    let paths = fs::read_dir("./tests/testfiles").unwrap();

    for path in paths {
        let src = fs::read_to_string(dbg!(path.unwrap().path().as_path())).expect("Failed to read test file");
        common::run(&src,"pass")
    }
}