use error_land::{err_from, err_struct, JsonLayer, PrettyLayer};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{fmt, fs};

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
fn main() -> Result<(), ErrorMain> {
    // let buffers = Arc::new(Mutex::new(Vec::new()));
    // PrettyLayer { buffers: buffers.clone() }
    let registry = tracing_subscriber::registry();
    match std::env::var("LOG_FORMAT") {
        Ok(format) if format == "json" => registry.with(JsonLayer).init(),
        _ => registry.with(PrettyLayer).init(), //.with(JsonLayer).init(),
    };

    _ = parse_single_float("./sample_float/valid.txt")?;

    _ = parse_single_float("./sample_float/invalid.txt").unwrap_or_else(|err| {
        warn!("{err}; Continue with default value");
        f64::default()
    });

    _ = parse_single_float("./sample_float/empty.txt").unwrap_or_else(|err| {
        warn!("{err}; Continue with default value");
        f64::default()
    });

    _ = parse_single_float("./sample_float/whitespace.txt").unwrap_or_else(|err| {
        warn!("{err}; Continue with default value");
        f64::default()
    });

    _ = parse_single_float("./sample_float/does_not_exist.txt")?;

    info!("We do not get here");
    Ok(())
}
