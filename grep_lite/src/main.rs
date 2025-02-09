use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;
use regex::Regex;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pattern: String,

    #[arg(short, long)]
    input: Option<String>,
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = Args::parse();
    let re = Regex::new(&args.pattern).unwrap();

    match args.input {
        Some(input) => {
            let f = File::open(input).unwrap();
            let reader = BufReader::new(f);
            process_lines(reader, re);

        },
        None => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, re);
        }
    }
}
