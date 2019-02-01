extern crate reqwest;
// #[macro_use]
// extern crate serde_derive;

// use reqwest::Error;
use std::io::copy;
use std::fs::File;
use std::str;
// use std::io::Result;

// #[derive(Deserialize)]
// struct Simple {
//    key: String
// }

fn make_request() -> Result<String, reqwest::Error> {
    let mut resp = reqwest::get("http://httpbin.org/range/something").expect("not able to make the request");
    // match make_request() {
    //    Err(e) => handler(e),
    // //    Ok(s)  => println!("result: {}", s),
    //    Ok(_)  => println!("this is good."),
    // };
    let mut resp1 = if resp.status().is_success() {
       resp
    } else {
        panic!("Something") // Great now this is working.
    };
    let mut buf: Vec<u8> = vec![];
    resp1.copy_to(&mut buf)?;
    let s = match str::from_utf8(&mut buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    // println!("result: {}", s);
    Ok(s.to_string())
}

fn handler(e: reqwest::Error) {
   if e.is_http() {
       match e.url() {
           None => println!("No Url given"),
           Some(url) => println!("Problem making request to: {}", url),
       }
   }
   // Inspect the internal error and output it
   if e.is_serialization() {
      let serde_error = match e.get_ref() {
           None => return,
           Some(err) => err,
       };
       println!("problem parsing information {}", serde_error);
   }
   if e.is_redirect() {
       println!("server redirecting too many times or making loop");
   }

   if e.is_client_error() {
       panic!("Server error {:#?}", e);
   }

}

fn main() -> Result<(), reqwest::Error> {
    // let target = "https://www.w3.org/TR/PNG/iso_8859-1.txt";
    // let mut response = reqwest::get(target)?;
    // let content = reqwest::get("http://httpbin.org/range/26")?.text()?;
    // let mut resp = reqwest::get("http://httpbin.org/range/26").expect("could not get the file.");
    // let mut resp = reqwest::get("http://httpbin.org/range/something")?;
    // let mut buf: Vec<u8> = vec![];
    // resp.copy_to(&mut buf);
    // let s = match str::from_utf8(&mut buf) {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };
    // println!("result: {}", s);
    // // assert_eq!(b"abcde", buf.as_slice());
    
    // let res_str = match make_request() {
    // match make_request() {
    //    Err(e) => handler(e),
    // //    Ok(s)  => println!("result: {}", s),
    //    Ok(_)  => println!("this is good."),
    // };
    // assert_eq!(b"abcde", res_str.as_slice());
    make_request();

    Ok(())
}