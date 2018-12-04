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
    let re = Regex::new(r"^.*Guard #(\d+) begins.*$").unwrap();
    let re2 = Regex::new(r"^.* 00:(\d{2}).*$").unwrap();
    let mut result = 0;
    for line in file.lines() {
        let l = line.unwrap();
        inputs.push(l.clone());
        // let caps = re.captures(&l).unwrap();
    }
    inputs.sort();
    let mut result = HashMap::new();
    // let mut result_max = HashMap::new();
    let mut result_time = HashMap::new();
    let mut iter = 0..inputs.len();
    loop {
        let mut i = match iter.next() {
            Some(x) => x,
            None => break,
        };
        let line = &inputs[i];
        if line.contains("begins") {
            let caps = re.captures(line).unwrap();
            // println!("{:?}", caps);
            let id = caps[1].parse::<i32>().unwrap();
            let mut tmp = 0;
            while i+2 < inputs.len() && inputs[i+1].contains("asleep") {
                let caps = re2.captures(&inputs[i+1]).unwrap();
                // println!("{:?}", caps);
                let start = caps[1].parse::<i32>().unwrap();
                let caps = re2.captures(&inputs[i+2]).unwrap();
                // println!("{:?}", caps);
                let end = caps[1].parse::<i32>().unwrap();
                let current = (end-start);
                tmp += current;
                if !result_time.contains_key(&id) {
                    result_time.insert(id, HashMap::new());
                }
                result_time.entry(id).and_modify(|e: &mut HashMap<i32, i32>| {
                    // println!("{}-{}-{}", id, start, end);
                    for j in start..end {
                        *e.entry(j).or_insert(0) += 1;
                    }
                });
                i += 2;
                iter.next();
                iter.next();
            }
            *result.entry(id).or_insert(0) += tmp;
            
        }
    }
    let mut max_asleep = 0;
    let mut maxid = 0;
    // println!("{:#?}, {:#?}", result, result_time);
    for (k, v) in result.iter() {
        if *v>max_asleep {
            max_asleep = *v;
            maxid = *k;
        }
    }
    let mut maxminute = 0;
    let mut maxtime = 0;
    for (k, v) in result_time.get(&maxid).unwrap().iter() {
        if *v>maxminute {
            maxminute = *v;
            // maxid = *k;
            maxtime = *k;
        }
    }
    println!("{}", maxid*maxtime);


    // part2
    let mut maxid2 = 0;
    let mut maxt = 0;
    let mut maxm = 0;
    for (id, time) in result_time {
        let mut tmpmaxt = 0;
        let mut tmpmaxm = 0;
        for (m, t) in time {
            if t>tmpmaxt {
                tmpmaxt = t;
                tmpmaxm = m;
            }
        }
        if tmpmaxt>maxt {
            maxt = tmpmaxt;
            maxid2 = id;
            maxm = tmpmaxm;
        }
    }
    println!("{}", maxid2*maxm);
}