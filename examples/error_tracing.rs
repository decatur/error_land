use error_land::{as_display, as_value, err_from, err_struct, into_err, JsonFormatter};
use serde_json::json;
use tracing::{error, info, warn, Level};

use std::process::{ExitCode, Termination};

err_struct!(ReadFileError);
fn read_file(path: &str) -> Result<String, ReadFileError> {
    let data = std::fs::read_to_string(path)?;
    if data.len() == 0 {
        Err(into_err("File was empty"))?
    } else if data.trim().len() == 0 {
        Err(into_err("File only has whitespace"))?
    } else {
        Ok(data)
    }
}

err_struct!(ReadFileError => ParseError);
fn parse_single_float(path: &str) -> Result<f64, ParseError> {
    info!(path, "Reading and parsing file");
    Ok(read_file(path)?.parse::<f64>()?)
}

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_line_number(true)
        .event_format(JsonFormatter { with_time: false })
        .init();

    match major() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            error!(error = as_display(err), termination = true);
            ExitCode::FAILURE
        }
    }
}

err_struct!(ParseError => ErrorMain);
fn major() -> Result<(), ErrorMain> {
    let doc = json!({
        "code": 200,
        "success": true,
        "payload": {
            "description": "foobar"
        }
    });

    _ = parse_single_float("./sample_float/valid.txt")?;

    _ = parse_single_float("./sample_float/invalid.txt").unwrap_or_else(|err| {
        let else_value = 1.;
        warn!(error = as_display(err), else_value, "Continue");
        else_value
    });

    _ = parse_single_float("./sample_float/empty.txt").unwrap_or_else(|err| {
        let else_value = 2.;
        warn!(error = as_display(err), "Continue \"with {}", else_value);
        else_value
    });

    _ = parse_single_float("./sample_float/whitespace.txt").unwrap_or_else(|err| {
        let else_value = 3.;
        error!(
            error = as_display(err),
            json = as_value(doc),
            "Continue with {}",
            else_value
        );
        else_value
    });

    _ = parse_single_float("./sample_float/does_not_exist.txt")?;

    unreachable!();
}
