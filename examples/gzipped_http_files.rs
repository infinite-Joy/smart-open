extern crate flate2;
extern crate reqwest;

use std::io::Read;
use std::result::Result;
use std::time::Duration;

use flate2::read::GzDecoder;
use reqwest::Client;

fn main() -> Result<(), reqwest::Error> {
    let resp = reqwest::get("https://wiki.mozilla.org/images/f/ff/Example.json.gz")?;
    let mut verified_response = if resp.status().is_success() {
        resp
    } else {
        panic!("The file not parsed correctly. Please check");
    };
    let mut buf: Vec<u8> = vec![];
    let _ = verified_response.copy_to(&mut buf)?;

    // Now going ahead with the gzip decoding.
    let mut gz = GzDecoder::new(&buf[..]);
    let mut s = String::new();
    match gz.read_to_string(&mut s) {
        Ok(v) => v,
        Err(e) => panic!("something not working"),
    };
    println!("final output:");
    println!("{}", s);
    Ok(())
}
