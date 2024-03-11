use std::error::Error;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1")
        .author("ozo")
        .about("rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("File name")
                .required(true)
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("Show number lines")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("Non blank lines")
                .takes_value(false)
        )
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}
