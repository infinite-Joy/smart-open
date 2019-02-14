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
        assert_eq!(
            sm::smart_open("s3://bml-data/churn-bigml-80.csv")
                .unwrap()
                .is_empty(),
            false,
            "not able to parse the remote file"
        );
    }

    #[test]
    #[should_panic(expected = r#"All the regions have been exhausted."#)]
    fn test_s3_nonexistingfile() {
        let _ = sm::smart_open("s3://bml-data/dummyfile.csv").unwrap();
    }

    #[test]
    fn test_open_http() {
        assert_eq!(
            sm::smart_open("http://httpbin.org/range/26").unwrap(),
            "abcdefghijklmnopqrstuvwxyz"
        );
    }

    #[test]
    #[should_panic(expected = r#"not parsed correctly"#)]
    fn test_http_non_existing_file() {
        let _ = sm::smart_open("http://httpbin.org/range/something").unwrap();
    }

    #[test]
    fn test_http_gzip_file() {
        assert_eq!(
            sm::smart_open("https://wiki.mozilla.org/images/f/ff/Example.json.gz").unwrap(), "{\"InstallTime\": \"1295768962\", \"Comments\": \"Will test without extension.\", \"Theme\": \"classic/1.0\", \"Version\": \"4.0b10pre\", \"id\": \"ec8030f7-c20a-464f-9b0e-13a3a9e97384\", \"Vendor\": \"Mozilla\", \"EMCheckCompatibility\": \"false\", \"Throttleable\": \"1\", \"Email\": \"deinspanjer@mozilla.com\", \"URL\": \"http://nighthacks.com/roller/jag/entry/the_shit_finally_hits_the\", \"version\": \"4.0b10pre\", \"CrashTime\": \"1295903735\", \"ReleaseChannel\": \"nightly\", \"submitted_timestamp\": \"2011-01-24T13:15:48.550858\", \"buildid\": \"20110121153230\", \"timestamp\": 1295903748.551002, \"Notes\": \"Renderers: 0x22600,0x22600,0x20400\", \"StartupTime\": \"1295768964\", \"FramePoisonSize\": \"4096\", \"FramePoisonBase\": \"7ffffffff0dea000\", \"AdapterRendererIDs\": \"0x22600,0x22600,0x20400\", \"Add-ons\": \"compatibility@addons.mozilla.org:0.7,enter.selects@agadak.net:6,{d10d0bf8-f5b5-c8b4-a8b2-2b9879e08c5d}:1.3.3,sts-ui@sidstamm.com:0.1,masspasswordreset@johnathan.nightingale:1.04,support@lastpass.com:1.72.0,{972ce4c6-7e08-4474-a285-3208198ce6fd}:4.0b10pre\", \"BuildID\": \"20110121153230\", \"SecondsSinceLastCrash\": \"810473\", \"ProductName\": \"Firefox\", \"legacy_processing\": 0}"
        );
    }
}
