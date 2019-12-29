extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod config;
mod schedule;

use std::io::Result;

fn main() -> Result<()> {
    //let mut fd = std::fs::File::open("foo.txt")?;
    let mut conf = config::Config::default();

    conf.backups.push(config::BackupConfig {
        src: "/foo/bar".into(),
        dst: "/bar/baz".into(),
        exclude_git_ignore: true,
    });

    let json = serde_json::to_string(&conf)?;
    println!("{}", json);

    Ok(())
}
