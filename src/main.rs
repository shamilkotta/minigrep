use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Parsing args failed: {err}");
        process::exit(1);
    }); 

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

struct Config {
    pattern: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
          return Err("Please provide pattern and file path!!");
        }

        let pattern = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            pattern,
            file_path,
            ignore_case,
        })    
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.pattern, &contents)
    } else {
        search(&config.pattern, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())

}