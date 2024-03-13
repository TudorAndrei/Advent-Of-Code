use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let mut output: Vec<i32> = data
        .trim_end()
        .split("\n\n")
        .map(|x| x.split("\n").collect())
        .collect::<Vec<Vec<&str>>>()
        .into_iter()
        .map(|x| x.into_iter().map(|y| y.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();
    output.sort();
    output.reverse();
    println!("{}", output[0]);
    let result2: i32 = output[0..=2].into_iter().sum();
    println!("{}", result2);
}
