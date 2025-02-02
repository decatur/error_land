use tracing::error as eprintln;

use error_vs::{
    err_struct, err_from
};
use std::fs;


err_struct!(ErrorC);
fn errors_c() -> Result<String, ErrorC> {
    let path = "./foo/bar.txt";
    let data = fs::read_to_string(path).map_err(|e| ErrorC(format!("{} {}", path, e)))?;
    if data.trim().len() == 0 {
        Err(ErrorC(format!("{} Was empty", path)))
    } else {
        Ok(data)
    }
}

err_struct!(ErrorC => ErrorB);
fn errors_b() -> Result<(), ErrorB> {
    let _data = errors_c()?;
    let _data = errors_c();
    Ok(())
}

err_struct!(ErrorB, ErrorC => ErrorA);
fn errors_a() -> Result<(), ErrorA> {
    errors_b()?;
    Ok(())
}

err_struct!(ErrorB => ErrorD);
fn errors_d() -> Result<(), ErrorD> {
    errors_b()?;
    errors_b()?;
    Ok(())
}

err_struct!(ErrorD => ErrorMain);
fn main() -> Result<(), ErrorMain> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("set tracing subscriber");

    if let Err(err) = errors_a() {
        println!("{:?}", err);
    }

    errors_d()?;
    Ok(())
}
