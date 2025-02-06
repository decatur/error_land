use error_vs::CustomLayer;
use tracing::{error,  info };
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use error_vs::{err_from, err_struct};
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

struct ParseOutput;
err_struct!(ReadFileError => ParseError);
fn parse_config(path: &str) -> Result<ParseOutput, ParseError> {
    info!("Reading and parsing {} ...", path);
    _ = read_file(path)?;
    // Do the parsing...
    Ok(ParseOutput)
}

err_struct!(ParseError => ErrorMain);
fn main() -> Result<(), ErrorMain> {
    // let subscriber = tracing_subscriber::fmt()
    //     .pretty()
    //     .with_target(false)
    //     .with_file(false)
    //     .with_line_number(false)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber)?;

    tracing_subscriber::registry().with(CustomLayer).init();

    _ = parse_config("./foo/bar.toml")?;
    Ok(())
}
