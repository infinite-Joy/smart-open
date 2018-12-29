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
    #[should_panic(expected = r#"png case is not implemented yet"#)]
    fn test_unknown_file() {
        let _ = sm::smart_open("tests/abc.png");
    }
}