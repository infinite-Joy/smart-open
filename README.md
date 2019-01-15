# smart-open
Opens files for reading.


## What??
`smart_open` is a rust crate under active development with the goal of opening any text file (compressed or uncompressed) present either in local or on the cloud or the web.

    extern crate smart_open as sm;
    
    pub fn main() {
        let text = sm::smart_open("tests/bar.txt.gz").unwrap();
        println!("{}", text);
    }

You can find sample code for other types of files in the `examples` directory.


## Goals for this project

- [x] Open text file on local filesystem.
- [x] Open gz text file on local filesystem.
- [x] Support for other text file formats (csv, json) etc.
- [x] Text files on s3.
- [ ] Gz files on s3.
- [ ] Files on http.


## Comments, bug reports
`smart_open` lives on Github. You can file issues or pull requests there. Suggestions, pull requests and improvements welcome!

`smart_open` is open source software released under the MIT license. Copyright (c) 2018-now Joydeep Bhattacharjee.
