use lncat::Config;

use std::env;

fn main() {
    let config_result = Config::from(env::args());

    match config_result {
        Ok(config) => {
            if let Err(error) = lncat::run(config) {
                println!("There was an error reading the file: {}", error);
            };
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
