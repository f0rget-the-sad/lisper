use lisper::Config;

use std::env;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    println!("{} version {}", NAME, VERSION);

    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error occurred during arg parsing\n{}", err);
        process::exit(1);
    });

    lisper::run(conf);
}
