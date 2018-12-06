extern crate regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let f = File::open("./src/test.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut inputs = vec![];
    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();
    let mut result = 0;
    let mut linenum = 0;
    let mut state = [[Point{
        Num: 0, 
        Distance: 100000
    }; 400]; 400];
    for i in 0..400 {
        for j in 0..400 {
            state[i][j] = Point{
                Num: 0, 
                Distance: 100000
            };
        }
    }
    let mut points = HashMap::new();
    // let mut skip = HashSet::new();
    for line in file.lines() {
        let l = line.unwrap();
        inputs.push(l.clone());
        linenum += 1;
        let caps = re.captures(&l).unwrap();
        let x = caps[1].parse::<i32>().unwrap();
        let y = caps[2].parse::<i32>().unwrap();
        points.insert(linenum, (x, y));
        for i in 0..400 {
            for j in 0..400 {
                let d = (x-i as i32).abs()+(y-j as i32).abs();
                if state[i][j].Distance > d {
                    state[i][j] = Point{Num: linenum, Distance: d};
                    // if i == 0 || j == 0 || i == 399 || j == 399 {
                    //     skip.push(linenum);
                    // }
                } else if state[i][j].Distance == d {
                    state[i][j] = Point{Num: 0, Distance: d};
                }
            }
        }
    }
    let mut newresult = [0i32; 401];
    for i in 0..400 {
        for j in 0..400 {
            let p = state[i][j];
            if p.Num == 0 {
                continue;
            }
            if newresult[p.Num as usize] == -1 {
                continue;
            }
            if i == 0 ||j ==0||i==399 ||j==399 {
                newresult[p.Num as usize] = -1;
                continue;
            }
            newresult[p.Num as usize] += 1;
        }
    }
    // for i in 0..10 {
    //     for j in 0..10 {
    //         print!("{:?} ", state[i][j].Num);
    //     }
    //     println!("");
    // }
    let mut max = 0;
    for i in 0..400 {
        // if i > linenum {
        //     continue;
        // }
        if newresult[i] > max {
            max = newresult[i];
        }
    }
    println!("{:?}", max);

    // println!("{:#?}", &points);
    // let dddd = 10000;
    let dddd = 32;
    let mut count = 0;
    for i in 0..400 {
        for j in 0..400 {
            let p = state[i][j];
            if p.Num == 0 {
                // continue;
            }
            if newresult[p.Num as usize] == -1 {
                // continue;
            }
            let mut tmp = 0;
            for (_, v) in &points {
                let (x, y) = v;
                tmp += (x-i as i32).abs()+(y-j as i32).abs();
                if tmp >= dddd {
                    break;
                }
            }
            if tmp < dddd {
                println!("{}-{}-{}", i, j, tmp);
                count += 1;
            }
        }
    }
    println!("{}", count);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    Num: i32,
    Distance: i32,
} 