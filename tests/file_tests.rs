use std::fs;

mod common;

#[test]
fn test() {
    let paths = fs::read_dir("./tests/testfiles").unwrap();

    for path in paths {
        let p = path.unwrap().path();
        if p.is_dir() { continue }
        println!("Executing script: {:?}",&p.as_path());
        let src = fs::read_to_string(p.as_path()).expect(&format!("Failed to read test file: {:?}", p));
        common::run(&src,"pass")
    }
}