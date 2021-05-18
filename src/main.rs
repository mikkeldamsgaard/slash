use std::io::{stdout, stderr, stdin, Read};
use std::process::exit;
use std::cell::RefCell;
use std::{env, fs};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cur_dir;
    let mut src = String::new();
    if args.len() == 1 {
        stdin().read_to_string(&mut src).expect("Could not read from stdin");
        cur_dir = env::current_dir().expect("Could not determine current dir");
    } else if args.len() == 2 {
        src = fs::read_to_string(&args[1]).expect(&format!("Failed to read file {}", &args[1]));
        cur_dir = Path::new(&args[1]).parent().expect("Failed to determine dir of input file").to_path_buf();
    } else {
        panic!("Could not parse command line args: {:?}",&args);
    }

    let res = slash::Slash::new(&src,
                                Box::new(RefCell::new(stdout())),
                                Box::new(RefCell::new(stderr())),
                                cur_dir.as_path(),
    ).run();

    match res {
        Ok(()) => exit(0),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

// TODO: Readme documentation
// TODO: More testing
// TODO: More standardlib function (table addressing, list addressing, exitcode)