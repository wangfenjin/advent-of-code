use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut result = 0;
    let mut frequency = HashMap::new();
    let mut inputs = vec![];
    frequency.insert(0, 1);
    for line in file.lines() {
        let l = line.unwrap();
        let i = l.parse::<i32>().unwrap();
        result += i;
        if frequency.contains_key(&result) {
            println!("{}", result);
            return;
        } else {
            frequency.insert(result, 1);
            inputs.push(i);
        }
    }
    println!("{}", result);
    while true {
        for i in inputs.iter() {
            result += i;
        if frequency.contains_key(&result) {
            println!("{}", result);
            return;
        } else {
            frequency.insert(result, 1);
            // inputs.push(i);
        }
        }
    }
    // println!("{}", result);
}
