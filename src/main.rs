extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod config;
use std::io::Result;
fn main() -> Result<()> {
    let mut fd = std::fs::File::open("foo.txt")?;
    config::Config::read(&mut fd)?;
    Ok(())
}
