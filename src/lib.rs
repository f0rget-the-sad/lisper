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
        let is_promt = file.len() == 0;

        Ok(Config { file, is_promt })
    }
}
