use error_land::{err_from, err_struct, JsonLayer, PrettyLayer};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::fs;

err_struct!(ReadFileError);
fn read_file(path: &str) -> Result<String, ReadFileError> {
    let data = fs::read_to_string(path)?;
    if data.trim().len() == 0 {
        Err(ReadFileError("File was empty".to_owned()))
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
    let registry = tracing_subscriber::registry();
    match std::env::var("LOG_FORMAT") {
        Ok(format) if format == "json" => registry.with(JsonLayer).init(),
        _ => registry.with(JsonLayer).init(), //registry.with(PrettyLayer).init(),
    };

    _ = parse_single_float("./float_valid.txt")?;

    _ = parse_single_float("./float_invalid.txt").unwrap_or_else(|_err| {
        warn!("Continue with default value");
        f64::default()
    });

    _ = parse_single_float("./do_not_exist.txt")?;

    info!("We do not get here");
    Ok(())
}
