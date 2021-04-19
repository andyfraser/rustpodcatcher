use std::error::Error;

const CONFIG_FILENAME: &str = "~/.config/podcatcher.conf";

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let filename = if args.len() > 1 {
            args[1].clone()
        } else {
            String::from(CONFIG_FILENAME)
        };

        Ok(Config {filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config.filename);

    Ok(())
}