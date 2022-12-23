use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_lines(fi_name: String) -> Vec<String> {
    let file = File::open(fi_name).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}