extern crate reqwest;

// inspiration https://github.com/cyplo/rust-dependency-injection/blob/master/src/main.rs

use reqwest::Error;
use std::io::Result;
use std::result::Result as StdRes;
use std::str;

trait Request {
    fn get(&self, url: &str) -> StdRes<reqwest::Response, Error>;
}

#[derive(Debug)]
struct SmartOpenRequest;

impl SmartOpenRequest {
    fn new() -> Self {
        SmartOpenRequest {}
    }
}

impl Request for SmartOpenRequest {
    fn get(&self, url: &str) -> StdRes<reqwest::Response, Error> {
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
struct SmartOpenRequestClient<'a, RequestType>
where
    RequestType: Request + 'a,
{
    request: &'a RequestType,
    response: Option<reqwest::Response>,
}

impl<'a, RequestType> SmartOpenRequestClient<'a, RequestType>
where
    RequestType: Request + 'a,
{
    fn with_request(request: &'a RequestType) -> Self {
        SmartOpenRequestClient {
            request,
            response: None,
        }
    }

    fn get_response(&mut self, url: &str) {
        println!("Going through the get response in SmartOpenRequestClient");
        self.response = Some(self.request.get(url).unwrap());
    }
}

// use super::*;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

#[derive(Debug)]
struct FakeRequest {
    req: String,
    move_by_secs: AtomicUsize,
}

impl FakeRequest {
    fn with_req(req: String) -> Self {
        FakeRequest {
            req,
            move_by_secs: AtomicUsize::new(0),
        }
    }
}

impl Request for FakeRequest {
    fn get(&self, url: &str) -> StdRes<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("lang", "rust");

        let client = reqwest::Client::new();
        let res = client.post("http://httpbin.org").json(&map).send()?;
        Ok(res)
    }
}

fn main() {
    let req = SmartOpenRequest::new();
    // let res = SmartOpenRequest::get("http://httpbin.org/range/26").unwrap();
    let mut tr = SmartOpenRequestClient::with_request(&req);
    tr.get_response("http://httpbin.org/range/26");
    match tr.response {
        Some(x) => println!("Response: {:#?}", x),
        None => println!("Nothing in the response."),
    }

    let req = FakeRequest::with_req("fake".to_string());
    println!("{:#?}", req);
    let mut client = SmartOpenRequestClient::with_request(&req);

    client.get_response("some url");
    match client.response {
        Some(x) => println!("cleitn restponse {:#?}", x),
        None => panic!("some problem"),
    }
}

// #[cfg(test)]
// mod should {

//     // use super::*;
//     use std::sync::atomic::AtomicUsize;
//     use std::sync::atomic::Ordering;
//     use std::time::Duration;

//     #[test]
//     fn handle_response() {
//         let req = FakeRequest::with_req(reqwest::get("some url"));
//         let mut client = SmartOpenRequestClient::with_request(&req);

//         client.get_response("some url");
//         assert_eq!(32, client.response);
//     }

//     fn time_difference_between_two_stored<RequestType>(
//         repository: TimestampingRepository<RequestType>,
//     ) -> Duration
//     where
//         RequestType: Request,
//     {
//         let stored_values = repository.all_stored();
//         let first_timestamp = stored_values[0].0;
//         let second_timestamp = stored_values[1].0;
//         second_timestamp - first_timestamp
//     }

//     struct FakeRequest {
//         get: Instant,
//         move_by_secs: AtomicUsize,
//     }

//     impl FakeRequest {
//         fn with_req(req: reqwest::Response) -> Self {
//             FakeRequest {
//                 req,
//                 move_by_secs: AtomicUsize::new(0),
//             }
//         }
//     }

//     impl Request for FakeRequest {
//         fn get(&self, url: &str) -> StdRes<reqwest::Response, Error> {
//             let mut map = HashMap::new();
//             map.insert("lang", "rust");

//             let client = reqwest::Client::new();
//             let res = client.post("http://httpbin.org").json(&map).send()?;
//             Ok(res)
//         }
//     }
// }
