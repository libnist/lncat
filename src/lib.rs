//! # Utils used to create a `cat` like command
//!  
//! Here I implemented a `Config` struct to set the configs and
//! a function called `run` to consume that struct.


use std::path::Path;
use std::io::{self, BufRead};
use std::fs::File;
use std::error::Error;

/// This struct contains all file paths to be concatenated
///
/// # Example
///
/// ```
/// use lncat::Config;
/// use std::env;
/// 
/// let config_result: Result<Config, String> = Config::from(env::args());
/// ```

pub struct Config {
    pub paths: Vec<String>,
}

impl Config {
    pub fn from(mut paths: impl Iterator<Item=String>) -> Result<Config, String> {

        // Ignore the first element
        paths.next();

        let mut result_paths: Vec<String> = Vec::new();
        for path in paths {

            let std_path = Path::new(&path);

            if std_path.is_file() {
                result_paths.push(path)
            } else if std_path.is_dir() {
                return Err(format!("`{}` is a directory", path))
            } else {
                return Err(format!("`{}` is an invalid input", path))
            }
        }

        Ok(Config {
            paths: result_paths,
        })
    }
}

/// The function `run` takes a `Config` as the input and concatenates all the files in it
/// 
/// # Example
/// ```
/// use lncat::Config;
/// use std::env;
/// 
/// let config_result = Config::from(env::args());
/// match config_result {
///     Ok(config) => {
///         lncat::run(config).expect("There was a problem reading file.");
///         },
///     Err(err) => {
///         eprintln!("{}", err);
///         }
/// }
/// ```

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for path in config.paths {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line?);
        }

    }

    Ok(())
}