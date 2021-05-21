use std::io::{stdout, stderr, stdin, Read};
use std::process::exit;
use std::cell::RefCell;
use std::{env, fs};
use std::path::Path;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let cur_dir;
    let mut src = String::new();

    /* let program = */ args.remove(0);

    if args.len() == 0 {
        stdin().read_to_string(&mut src).expect("Could not read from stdin");
        cur_dir = env::current_dir().expect("Could not determine current dir");
    } else if args.len() >= 1 {
        let script = &args[0];

        src = fs::read_to_string(script).expect(&format!("Failed to read file {}", script));
        cur_dir = Path::new(script).parent().expect("Failed to determine dir of input file").to_path_buf();
    } else {
        panic!("Could not parse command line args: {:?}",&args);
    }

    let res = slash::Slash::new(&src,
                                Box::new(RefCell::new(stdout())),
                                Box::new(RefCell::new(stderr())),
                                cur_dir.as_path(),
                                 args
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