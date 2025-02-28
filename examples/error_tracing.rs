use error_land::{err_from, err_struct, JsonFormatter, PrettyFormatter};
use serde_json::json;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

use std::{fmt, fs, process::Termination};

#[derive(Debug)]
struct Error(String);

impl Error {
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

err_struct!(ReadFileError);
fn read_file(path: &str) -> Result<String, ReadFileError> {
    let data = fs::read_to_string(path)?;
    if data.len() == 0 {
        Err(Error::new("File was empty"))?
    } else if data.trim().len() == 0 {
        Err(Error::new("File only has whitespace"))?
    } else {
        Ok(data)
    }
}

err_struct!(ReadFileError => ParseError);
fn parse_single_float(path: &str) -> Result<f64, ParseError> {
    info!(path, "Reading and parsing file");
    Ok(read_file(path)?.parse::<f64>()?)
}

err_struct!(ParseError => ErrorMain);

struct T(Option<ErrorMain>);

impl Termination for T {
    fn report(self) -> std::process::ExitCode {
        match self.0 {
            Some(err) => {
                error!(error = err.to_error(), termination = true);
                std::process::ExitCode::FAILURE
            }
            None => std::process::ExitCode::SUCCESS,
        }
    }
}

fn main() -> T {
    match major() {
        Ok(_) => T(None),
        Err(err) => T(Some(err)),
    }
}

fn major() -> Result<(), ErrorMain> {
    let layer = tracing_subscriber::fmt::layer();
    match std::env::var("LOG_FORMAT") {
        Ok(format) if format == "json" => Registry::default()
            .with(layer.event_format(JsonFormatter))
            .init(),
        _ => Registry::default()
            .with(layer.event_format(PrettyFormatter))
            .init(),
    };

    let doc = json!({
        "code": 200,
        "success": true,
        "payload": {
            "msg": "foobar"
        }
    });

    _ = parse_single_float("./sample_float/valid.txt")?;

    _ = parse_single_float("./sample_float/invalid.txt").unwrap_or_else(|err| {
        let else_value = 1.;
        warn!(error = err.to_error(), "Continue with {}", else_value);
        else_value
    });

    _ = parse_single_float("./sample_float/empty.txt").unwrap_or_else(|err| {
        let else_value = 2.;
        warn!(error = err.to_error(), "Continue \"with {}", else_value);
        else_value
    });

    _ = parse_single_float("./sample_float/whitespace.txt").unwrap_or_else(|err| {
        let else_value = 3.;
        error!(error = err.to_error(), json = %doc, "Continue with {}", else_value);
        else_value
    });

    _ = parse_single_float("./sample_float/does_not_exist.txt")?;

    unreachable!();
}
