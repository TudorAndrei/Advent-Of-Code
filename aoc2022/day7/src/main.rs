mod fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use fs::Dir;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    let fs = Dir::new("/".to_string());
    if let Ok(lines) = read_lines("input/test.in") {
        for line in lines {
            match line {
                Ok(line) => {
                    if line.starts_with("$") {
                        let a: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
                        if a[1] == "cd" {
                            println!("{:?}", a[2]);
                        }
                        println!("{:?}", line);
                    }
                }
                Err(_) => println!("Unable to read line"),
            }
        }
    }
}
