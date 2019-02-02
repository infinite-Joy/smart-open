extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate reqwest;

#[derive(Deserialize)]
struct Simple {
   key: String
}

fn main() {
   match make_request() {
       Err(e) => handler(e),
       Ok(_)  => return,
   }
}
// Response is not a json object conforming to the Simple struct
// fn make_request() -> Result<reqwest::Response, reqwest::Error> {
fn make_request() -> reqwest::Response {
//   reqwest::get("http://httpbin.org/ip")?.json()
//   reqwest::get("http://httpbin.org/ip")?
  let mut resp =  reqwest::get("http://httpbin.org/range/something").expect("Failed to send request");
  if resp
}

fn handler(e: reqwest::Error) {
//    if e.is_http() {
//        match e.url() {
//            None => println!("No Url given"),
//            Some(url) => println!("Problem making request to: {}", url),
//        }
//    }
//    // Inspect the internal error and output it
//    if e.is_serialization() {
//       let serde_error = match e.get_ref() {
//            None => return,
//            Some(err) => err,
//        };
//        println!("problem parsing information {}", serde_error);
//    }
//    if e.is_redirect() {
//        println!("server redirecting too many times or making loop");
//    }
   if e.is_client_error() {
       panic!("Not able to parse the resource.")
   }
}