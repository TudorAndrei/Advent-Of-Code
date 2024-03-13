use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("test").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let output: Vec<i32> = data
        .trim_end()
        .lines()
        .map(|x| match x.parse::<i32>().unwrap() {
            Some(x) => x,
            None => 0,
        })
        .collect::<Vec<i32>>();
    for o in output {
        println!("{}", o);
    }
}
