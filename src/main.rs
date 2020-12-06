use lisper::Config;
use std::ops::Add;

use std::env;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Copy, Clone)]
enum Numbers {
    Int(i64),
    Float(f64),
}

impl Add for Numbers {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        number_exec_op(self, other, &Add::add)
        /*
        let types = (self, other);
        match types {
            (Numbers::Int(x), Numbers::Int(y)) => Self::Int(
                x + y),
            (Numbers::Int(x), Numbers::Float(y)) => Self::Float(
                x as f64 + y),
            (_,_) => panic!("fuck")
        }
        */
    }
}

fn number_exec_op<T>(x1: Numbers, x2: Numbers, f: &dyn Fn(T, T) -> T) -> Numbers {
    Numbers::Int(1)
}

fn main() {
    println!("{} version {}", NAME, VERSION);

    let x = Numbers::Int(52);
    let xx = Numbers::Int(48);
    let y = Numbers::Float(5.2);

    let x1 = 52;
    let x2 = 5.2;

    println!("{}", (x1 as f64).min(x2));
    println!("{}", x1 as f64 - x2);
    println!("{}", Add::add(x1, 10));
    println!("{:?}", x + xx);
    println!("{:?}", x + y);

    //process::exit(1);

    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error occurred during arg parsing\n{}", err);
        process::exit(1);
    });

    lisper::run(conf);
}
