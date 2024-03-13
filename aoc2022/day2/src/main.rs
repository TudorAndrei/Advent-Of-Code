use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("test").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let mut choices: (&str, &str) = data
        .trim_end()
        .split("\n\n")
        .map(|x| x.split(" ").collect());
}
