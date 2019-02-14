extern crate reqwest;

// inspiration https://github.com/cyplo/rust-dependency-injection/blob/master/src/main.rs

use reqwest::Error;
use std::io::Result;
use std::result::Result as StdRes;
use std::str;

trait Request {
    fn get(url: &str) -> StdRes<reqwest::Response, Error>;
}

#[derive(Debug)]
struct SmartOpenRequest;

impl Request for SmartOpenRequest {
    fn get(url: &str) -> StdRes<reqwest::Response, Error> {
        let r = reqwest::get(url)?;
        Ok(r)
    }
}

// fn main() {
//     let resp = SmartOpenRequest::get("http://httpbin.org/range/26").unwrap();
//     let verified_response = if resp.status().is_success() {
//         resp
//     } else {
//         panic!("The file not parsed correctly. Please check");
//     };
//     println!("{:#?}", verified_response);
//     // let mut buf: Vec<u8> = vec![];
//     // res.copy_to(&mut buf);
//     // let s = match str::from_utf8(&buf) {
//     //     Ok(v) => v,
//     //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//     // };
//     // println!("{}", s);
// }

#[derive(Debug)]
struct TimestampingRepository<'a, RequestType>
where
    RequestType: Request + 'a,
{
    url: &'a RequestType,
}

impl<'a, RequestType> TimestampingRepository<'a, RequestType>
where
    RequestType: Request + 'a,
{
    // fn get(url: &'a RequestType) -> StdRes<reqwest::Response, Error> {

    // }
}

fn main() {
    let res = SmartOpenRequest::get("http://httpbin.org/range/26").unwrap();
    let mut repository = TimestampingRepository::get(&res);
}

#[cfg(test)]
mod should {

    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    struct FakeRequest {
        get: Instant,
    }

}
