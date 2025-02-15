use error_landscape::JsonLayer;
use tracing::{error,  info };
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use anyhow::Result;
use std::fs::{self};



#[track_caller]
#[inline(always)]
fn log_error(msg: &str) {
    let caller = std::panic::Location::caller();
    error!(caller = caller.to_string(), message=msg);
}

fn read_file(path: &str) -> Result<String> {
    let data = fs::read_to_string(path).inspect_err(|e| log_error(&e.to_string()))?;
    if data.trim().len() == 0 {
        log_error("File was empty");
        Err(anyhow::Error::msg("File was empty"))
    } else {
        Ok(data)
    }
}

struct ParseOutput;
fn parse_config(path: &str) -> Result<ParseOutput> {
    info!("Reading and parsing {} ...", path);
    _ = read_file(path).inspect_err(|e| log_error(&e.to_string()));
    // Do the parsing...
    Ok(ParseOutput)
}

fn main() -> Result<()> {
    tracing_subscriber::registry().with(JsonLayer).init();

    _ = parse_config("./foo/bar.toml")?;
    Ok(())
}
