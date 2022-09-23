use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let file = File::open("img/text.txt").expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut str=String::new();
    reader.read_line(&mut str).expect("Unable to read file");
    println!("{}",str);
}
