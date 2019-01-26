extern crate smart_open as sm;

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_text_file() {
        assert_eq!(sm::smart_open("tests/foo.txt").unwrap(), "Hello, world!");
    }

    #[test]
    #[should_panic(expected = r#"without extension"#)]
    fn test_gzip_file_invalidfile() {
        let _ = sm::smart_open("somefile");
    }

    #[test]
    fn test_gzip_file() {
        assert_eq!(sm::smart_open("tests/bar.txt.gz").unwrap(), "Hello, world!");
    }

    #[test]
    fn test_s3_simple_file() {
        assert_eq!(sm::smart_open("s3://bml-data/churn-bigml-80.csv").unwrap().is_empty(),
        false, "not able to parse the remote file");
    }

    #[test]
    #[should_panic(expected = r#"All the regions have been exhausted."#)]
    fn test_s3_nonexistingfile() {
        let _ = sm::smart_open("s3://bml-data/dummyfile.csv").unwrap();
    }

    #[test]
    fn test_open_http() {
        // assert_eq!(sm::smart_open("http://httpbin.org/range/26").unwrap().is_empty(),
        // false, "not able to parse the remote file");
        assert_eq!(sm::smart_open("http://httpbin.org/range/26").unwrap(),
        "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    // #[should_panic(expected = r#"All the regions have been exhausted."#)]
    fn test_http_non_existing_file() {
        assert_eq!(sm::smart_open("http://httpbin.org/range/something").unwrap(), "abcdefghijklmnopqrstuvwxyz");
    }
}
