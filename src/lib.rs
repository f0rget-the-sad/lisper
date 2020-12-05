extern crate pest;
#[macro_use]
extern crate pest_derive;

mod error;
mod evaluator;
mod parser;

use rustyline::{error::ReadlineError, Editor};

const HISTORY_PATH: &str = "/tmp/lister_history.txt";

#[derive(Debug)]
pub struct Config {
    pub file: String,
    pub is_promt: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // skip program name
        args.next();

        let file = match args.next() {
            Some(arg) => arg,
            None => "".to_string(),
        };
        let is_promt = file.is_empty();

        Ok(Config { file, is_promt })
    }
}

pub fn run(conf: Config) {
    if conf.is_promt {
        promt();
    } else {
        proc_file(conf.file);
    }
}

fn proc_file(file_name: String) {
    println!("Processing file '{}'...", file_name);
}

fn promt() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(HISTORY_PATH).is_err() {
        // TODO: add logger and DEBUG info
        //println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let line_str = line.as_str();
                rl.add_history_entry(line_str);
                match parser::parse_and_eval(line_str) {
                    Ok(r) => println!("{} => {}", line_str, r),
                    Err(e) => println!("Error: {}", e),
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C, exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D, exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HISTORY_PATH).unwrap();
}
