extern crate getopts;

use std::env;

pub struct Args {
    pub query: String,
    pub is_help: bool,
}

pub fn parse_args() -> Option<Args> {
    let input_args: Vec<String> = env::args().collect();
    let program = input_args[0].clone();

    let mut options = getopts::Options::new();
    options.optflag("h", "help", "print this help menu");
    options.optflagopt("s", "search", "Search query", "SEARCH");

    let matches = match options.parse(&input_args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            print_usage(&program, options);
            return None;
        }
    };

    let is_help = matches.opt_present("h");
    if is_help {
        print_usage(&program, options);
    }

    let query = match matches.opt_str("search") {
        Some(x) => x.clone(),
        None => "".to_string(),
    };

    Some(Args {
        query: query,
        is_help: is_help
    })
}

fn print_usage(program: &str, options: getopts::Options) {
    let brief = format!("Usage: {:?} [options]", program);
    print!("{}", options.usage(&brief));
}
