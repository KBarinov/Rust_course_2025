#![forbid(unsafe_code)]
use ::std::collections::HashSet;
use std::{fs::File, io::BufRead, io::BufReader};

fn search_file(file1: File, file2: File) -> HashSet<String> {
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);
    let mut line = HashSet::new();
    for line1 in reader1.lines() {
        let line1_ = line1.unwrap();
        line.insert(line1_.clone());
    }
    let mut coincidence = HashSet::new();
    for line2 in reader2.lines() {
        let line2_ = line2.unwrap();
        if line.contains(&line2_) {
            coincidence.insert(line2_.clone());
        }
    }
    coincidence
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let file1 = File::open(&args[1]).unwrap();
    let file2 = File::open(&args[2]).unwrap();
    let coincidence = search_file(file1, file2);
    for line in &coincidence {
        println!("{line}");
    }
}
