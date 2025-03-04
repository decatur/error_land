use core::fmt;
use std::process::ExitCode;

struct ErrorString(String);

impl fmt::Display for ErrorString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<E: std::error::Error> From<E> for ErrorString {
    fn from(e: E) -> Self {
        Self(e.to_string())
    }
}

type Result<T> = std::result::Result<T, ErrorString>;

fn read_file(path: &str) -> Result<String> {
    let data = std::fs::read_to_string(path)?;
    if data.len() == 0 {
        Err(ErrorString("File was empty".to_owned()))?
    } else if data.trim().len() == 0 {
        Err(ErrorString("File only has whitespace".to_owned()))?
    } else {
        Ok(data)
    }
}

fn parse_single_float(path: &str) -> Result<f64> {
    println!("Reading and parsing file {path}");
    Ok(read_file(path)?.parse::<f64>()?)
}

fn main() -> ExitCode {
    match major() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn major() -> Result<()> {
    _ = parse_single_float("./sample_float/valid.txt")?;

    _ = parse_single_float("./sample_float/invalid.txt").unwrap_or_else(|err| {
        let else_value = 1.;
        eprintln!("{err}; Continue with {else_value}");
        else_value
    });

    _ = parse_single_float("./sample_float/empty.txt").unwrap_or_else(|err| {
        let else_value = 2.;
        eprintln!("{err}; Continue with {else_value}");
        else_value
    });

    _ = parse_single_float("./sample_float/whitespace.txt").unwrap_or_else(|err| {
        let else_value = 3.;
        eprintln!("{err}; Continue with {else_value}");
        else_value
    });

    _ = parse_single_float("./sample_float/does_not_exist.txt")?;

    unreachable!();
}
