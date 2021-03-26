use std::error::Error;

pub struct Config {
    pub test: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let test = if args.len() > 1 {
            args[1].clone()
        } else {
            String::new()
        };

        Ok(Config {test})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config.test);

    Ok(())
}