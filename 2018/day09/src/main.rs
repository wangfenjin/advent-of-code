extern crate regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut inputs = vec![];
    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    // let mut result = 0;
    let mut linenum = 0;
    // let mut state = [[0i32; 400]; 400];
    for line in file.lines() {
        let l = line.unwrap();
        inputs.push(l.clone());
        linenum += 1;
        let caps = re.captures(&l).unwrap();
        let players = caps[1].parse::<i32>().unwrap();
        let to = caps[2].parse::<i64>().unwrap();
        println!("{}-{}", players, to);

        let mut left = vec![];
        let mut right = VecDeque::new();
        left.push(0);
        // re.push(1);
        // let mut current = 0;
        let mut worth = 0 as i64;
        let mut result = HashMap::with_capacity(players as  usize);
        loop {
            if worth > to {
                break;
            }
            for i in 1..=players {
                worth += 1;
                if worth > to {
                    break;
                }
                if worth%23 == 0 {
                    // println!("{}-{}-{:?}-{:?}", i, current, re, result);
                    *result.entry(i).or_insert(0 as i64) += worth;
                    let mut tmp = Vec::with_capacity(8);
                    // println!("{:?}-{:?}-{:?}", left, right, result);
                    for i in 0..8 {
                        if left.len()>0 {
                            tmp.push(left.pop().unwrap());
                        } else {
                            tmp.push(right.pop_back().unwrap());
                        }
                    }
                    let t = tmp.pop().unwrap();
                    left.push(tmp.pop().unwrap());
                    tmp.reverse();
                    while let Some(a) = tmp.pop() {
                        right.push_front(a);
                    }
                    // right = tmp;
                    result.entry(i).and_modify(|e| {
                        // let t = left.pop().unwrap();
                        // println!("hit-{}", t);
                        *e += t;
                    });
                    // println!("{}-{:?}-{:?}-{:?}", t, left, right, result);
                    continue;
                }
                if right.len() == 0 {
                    let leftlen = left.len();
                    right = left.drain(0..leftlen).collect();
                }
                left.push(right.pop_front().unwrap());
                left.push(worth);
                // println!("{:?}-{:?}-{:?}", left, right, result);
                // println!("{}-{}-{:?}-{:?}", i, current, re, result);
            }
        }
        let mut max = 0;
        for (k,v) in result {
            if v>max {
                max = v;
            }
        }
        println!("{}", max);
    }
    
}
