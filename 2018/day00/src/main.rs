use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut result = 0;
    for line in file.lines() {
        let l = line.unwrap();
        let i = l.parse::<i32>().unwrap();
        result += i;
    }  
    println!("{}", result);
}
