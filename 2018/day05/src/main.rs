extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut inputs = vec![];
    for line in file.lines() {
        let l = line.unwrap();
        for c in l.chars() {
            inputs.push(c as u8);
        }
        let (mut min, result) = short(&inputs, 0);
        println!("{}", min);
        for j in 65..91 {
            let (llen, result) = short(&result, j as u8);
            if llen < min {
                min = llen;
            }
        }
        println!("{}", min);
    }
}

fn short(input: &Vec<u8>, j: u8) -> (usize, Vec<u8>) {
    let mut result = input.clone();
    let mut find = true;
    while find {
        find = false;
        let llen = result.len();
        if llen < 2 {
            break;
        }
        let mut tmp = vec![];
        let mut iter = 0..llen;
        loop {
            let i = match iter.next() {
                Some(x) => x,
                None => break,
            };
            if j>0 && (result[i]==j ||result[i]==j+32) {
                find = true;
            } else if i + 1 < llen
                && ((result[i] > result[i + 1] && result[i] - result[i + 1] == 32)
                    || (result[i] < result[i + 1] && result[i + 1] - result[i] == 32))
            {
                iter.next();
                find = true;
            } else {
                tmp.push(result[i].clone());
            }
        }
        result = tmp;
    }
    (result.len(), result.clone())
}