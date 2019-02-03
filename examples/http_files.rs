// $ cargo run --example http_files
//    Compiling smart_open v0.1.2 (/home/saionee/opensource/smart-open)
//     Finished dev [unoptimized + debuginfo] target(s) in 28.78s
//      Running `target/debug/examples/http_files`
// abcdefghijklmnopqrstuvwxyz

extern crate smart_open as sm;

pub fn main() {
    let text = sm::smart_open("http://httpbin.org/range/26").unwrap();
    println!("{}", text);
}
