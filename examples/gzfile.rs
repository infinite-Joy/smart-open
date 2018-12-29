extern crate smart_open as sm;

pub fn main() {
    let text = sm::smart_open("tests/bar.txt.gz").unwrap();
    println!("{}", text);
}
