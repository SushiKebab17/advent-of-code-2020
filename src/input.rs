use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn input(name: &str) -> Vec<String> {
    BufReader::new(File::open(format!("{}{}{}", "inputs/", name, ".txt")).unwrap())
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .unwrap()
}
