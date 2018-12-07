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
    let re = Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
    let mut result = 0;
    let mut linenum = 0;
    let mut state = [[0i32; 400]; 400];
    let mut du = HashMap::new();
    let mut dir = HashMap::new();
    for line in file.lines() {
        let l = line.unwrap();
        inputs.push(l.clone());
        linenum += 1;
        let caps = re.captures(&l).unwrap();
        let from = caps[1].chars().next().unwrap();
        let to = caps[2].chars().next().unwrap();
        du.entry(from.clone()).or_insert(0);
        *du.entry(to.clone()).or_insert(0) += 1;
        (*dir.entry(from.clone()).or_insert(vec![])).push(to.clone());
    }
    let mut result = vec![];
    let start = 'A' as u8 - 61;

    let mut works = HashMap::new();
    let mut time =0;
    loop {
        if du.len() == 0 {
            break;
        }
        let mut zero = vec![];
        for (k, v) in &du {
            if *v == 0 {
                zero.push(k.clone());
                // du.remove(k);
            }
        }
        zero.sort();
        for z in zero {
            if works.len()==5 {
                break;
            }
            works.entry(z.clone()).or_insert(z as i32-start as i32);
        }
        let mut min = 10234i32;
        let mut mina = 'A';
        for (k,v) in &works {
            if *v<min {
                min = *v;
                mina = *k;
            } else if *v==min && *k<mina {
                mina = *k;
            }
        }
        works.remove(&mina);
        for (_, val) in works.iter_mut() {
            *val -= min;
        }
        time += min;
        result.push(mina);
        du.remove(&mina);
        if !dir.contains_key(&mina) {
            continue;
        }
        let tos = dir.get(&mina).unwrap();
        for to in tos {
            du.entry(to.clone()).and_modify(|e| *e = *e - 1);
        }
    }
    let s: String = result.into_iter().collect();
    println!("{:?} {}", s, time);
}
