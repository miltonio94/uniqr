use clap::{App, Arg};
use std::{
    error::Error,
    fmt::format,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<R> = Result<R, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    input_file: String,
    output_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("M")
        .about("Rust uniq")
        .arg(
            Arg::with_name("input_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("output_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .help("Show count")
                .short("c")
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        input_file: matches.value_of("input_file").unwrap_or("-").to_string(),
        output_file: matches.value_of("output_file").map(|s| s.to_string()),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.input_file).map_err(|e| format!("{}: {}", config.input_file, e))?;
    let mut line = String::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
