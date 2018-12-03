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
    let mut inputs = vec![];
    // #3 @ 5,5: 2x2
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut result = 0;
    let mut state = [[0i32; 1024]; 1024];
    for line in file.lines() {
        let l = line.unwrap();
        inputs.push(l.clone());
        let caps = re.captures(&l).unwrap();
        // println!("{:?}", caps);
        let id = caps[1].parse::<i32>().unwrap();
        let left = caps[2].parse::<i32>().unwrap();
        let top = caps[3].parse::<i32>().unwrap();
        let width = caps[4].parse::<i32>().unwrap();
        let height = caps[5].parse::<i32>().unwrap();
        for i in left..left+width {
            for j in top..top+height {
                if state[i as usize][j as usize] == 1 {
                    result += 1;
                }
                state[i as usize][j as usize] += 1;
            }
        }
    }
    println!("{}", result);
    for line in inputs.iter() {
        let caps = re.captures(&line).unwrap();
        // println!("{:?}", caps);
        let id = caps[1].parse::<i32>().unwrap();
        let left = caps[2].parse::<i32>().unwrap();
        let top = caps[3].parse::<i32>().unwrap();
        let width = caps[4].parse::<i32>().unwrap();
        let height = caps[5].parse::<i32>().unwrap();
        let mut overlapped = false;
        for i in left..left+width {
            for j in top..top+height {
                if state[i as usize][j as usize] > 1 {
                    overlapped = true;
                    break;
                }
            }
        }
        if !overlapped {
            println!("{}", id);
            return;
        }
    }
}