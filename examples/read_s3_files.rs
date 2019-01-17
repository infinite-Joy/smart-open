extern crate smart_open as sm;

pub fn main() {
    let fp = "s3://bml-data/churn-bigml-80.csv";
    let text = sm::smart_open(&fp).unwrap();
    println!("{}", text);
}
