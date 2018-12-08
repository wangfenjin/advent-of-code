extern crate regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    // let mut inputs = vec![];
    let re = Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
    let mut result = 0;
    let mut linenum = 0;
    let mut state = [[0i32; 400]; 400];
    for line in file.lines() {
        let l = line.unwrap();
        // 2 3 [0 3 10 11 12 1 1 0 1 99 2] 1 1 2
        let v: Vec<&str> = l.split(' ').collect();
        let mut v = v.iter().map(|e| e.parse::<u32>().unwrap()).collect();
        println!("{:?}", tr(&v, 0, v.len()));
    }
}

fn tr(v: &Vec<u32>, start: usize, end: usize) -> (u32, usize, u32) {
    // println!("{}-{}", start, end);
    if end-start <= 2 {
        return (0, end, 0);
    }
    let mut sum = 0;
    let child = v[start];
    let entry = v[start+1] as usize;
    if child == 0 {
        for i in start+2..start+2+entry {
            sum += v[i];
        }
        return (sum, start+2+entry, sum);
    }
    let mut childstart = start+2;
    let mut value = 0;
    if child == 1 {
        let (tmp, child_end, tmp_value) = tr(v, childstart, end);
        sum += tmp;
        for i in child_end..child_end+entry {
            sum += v[i];
            if v[i] == 1 {
                value += tmp_value;
            }
        }
        return (sum, child_end+entry, value);
    }
    let mut child_value = HashMap::new();
    for i in 0..child {
        let (tmp, child_end, tmp_value) = tr(v, childstart, end);
        sum += tmp;
        childstart = child_end;
        child_value.insert(i+1, tmp_value);
    }
    for i in childstart..childstart+entry {
        sum += v[i];
        if child_value.contains_key(&v[i]) {
            value += child_value.get(&v[i]).unwrap();
        }
    }
    return (sum, childstart+entry, value);
}