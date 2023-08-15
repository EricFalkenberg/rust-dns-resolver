extern crate byte_string;
extern crate hex;

mod header;
mod question;
mod record;
mod util;
mod rfc_type;
mod result;
mod dns;

use std::env;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let default = String::from("example.com");
    let domain = args.get(1).unwrap_or(&default);
    println!("Querying DNS for: {0}", domain);
    let results = dns::resolve(domain)?;
    println!("{:#?}", results);
    Ok(())
}