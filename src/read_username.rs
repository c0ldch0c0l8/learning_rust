use std::fs::File;
use std::io::Read;

fn read_username(filename: &str) -> Result<String, std::io::Error> {
    let mut s = String::new();

    File::open(filename)?.read_to_string(&mut s)?;

    Ok(s)
}