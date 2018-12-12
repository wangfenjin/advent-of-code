extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::{thread, time};

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut inputs = vec![];
    let re = Regex::new(r"^initial state: (.*)$").unwrap();
    let re2 = Regex::new(r"^(.*) => (.)$").unwrap();
    // let mut points = vec![];
    let mut linenum = 0;
    let mut maxx = -1000;
    let mut maxy = -1000;
    let mut minx = 1000;
    let mut miny = 1000;

    let one_sec = time::Duration::from_millis(1000);

    let mut input = String::new();
    let mut m = HashMap::new();
    // let mut state = [[0i32; 400]; 400];
    for line in file.lines() {
        let l = line.unwrap();
        linenum += 1;
        if l.len() == 0 {
            continue;
        }
        inputs.push(l.clone());
        if linenum == 1 {
            let caps = re.captures(&l).unwrap();
            input = format!("....{}........", caps[1].to_string());
        } else if linenum > 2 {
            let caps = re2.captures(&l).unwrap();
            // input = caps[1];
            if caps[2].to_string() == "#".to_string() {
                m.insert(caps[1].to_string(), true);
                // println!("{}", caps[1].to_string());
            }
        }
    }
    println!("{:?}", m);
    println!("{}: {}", 0, input);
    let mut pre = 0;
    for j in 0..200 {
        let mut result = HashMap::new();
        for i in 2..input.len() - 2 {
            // let key = if i == 0 {
            //     format!("..{}", &input[i..(i+3)])
            // } else if i == 1 {
            //     format!(".{}", &input[(i-1)..(i+3)])
            // } else {
            //     input[(i-2)..(i+3)].to_string()
            // };
            let index = input.get(i..i + 1).unwrap();
            let target = if m.contains_key(&input[(i - 2)..(i + 3)]) {
                "#"
            // println!("{}: {}", &input[(i-2)..(i+3)], true);
            } else {
                "."
                // println!("{}: {}", &input[(i-2)..(i+3)], false);
            };
            if !index.contains(target) {
                if target == "#" {
                    result.insert(i, '#');
                } else {
                    result.insert(i, '.');
                }
            }
        }

        let mut tmp = input.into_bytes();
        for (k, v) in result {
            tmp[k] = v as u8;
        }
        // input = String::from_utf8(tmp).unwrap();
        // let tmp = result.into_iter().collect::<String>();
        input = String::from_utf8(tmp).unwrap();
        if input.get((input.len() - 4)..).unwrap() != "...." {
            input.push('.');
            input.push('.');
            input.push('.');
            input.push('.');
        }
        let mut sum = 0;
        for i in 2..input.len() - 1 {
            if input.get(i..i + 1).unwrap() == "#" {
                sum += (i as i64 - 4);
            }
        }
        // println!("{}", sum);
        println!("{}: {}, {}", j, sum, sum-pre);
        pre = sum;
    }
    // >>> 4107+23*(50000000000-163)
}

#[derive(Debug, Clone, Copy)]
struct Point {
    X: i32,
    Y: i32,
    Vx: i32,
    Vy: i32,
}
