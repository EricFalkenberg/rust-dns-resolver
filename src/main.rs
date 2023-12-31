extern crate byte_string;
extern crate hex;

mod header;
mod question;
mod record;
mod util;
#[allow(clippy::upper_case_acronyms)]
mod rfc_type;
mod result;
mod dns;

use std::env;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let default = String::from("example.com");
    let domain = args.get(1).unwrap_or(&default);
    let results = dns::resolve(domain)?;
    results.print_records();
    Ok(())
}