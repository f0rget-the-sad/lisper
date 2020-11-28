use lisper::Config;
use rustyline::{error::ReadlineError, Editor};
use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const HISTORY_PATH: &'static str = "/tmp/lister_history.txt";

fn main() {
    println!("{} version {}", NAME, VERSION);

    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error occurred during arg parsing\n{}", err);
        process::exit(1);
    });

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
        //println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
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
