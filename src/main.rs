use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("examples/example-1.wy");

    // open file
    if let file = File::open(&path) {

        // read lines
        let buf = BufReader::new(file);

        for line in buf.lines() {
            if let Ok(s) = line {
                println!("'{}'", s)
            }
        }
    }
}
