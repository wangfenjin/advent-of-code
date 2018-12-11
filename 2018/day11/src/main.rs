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
    // let mut inputs = vec![];
    let re = Regex::new(r"^position=<(.*)> velocity=<(.*)>$").unwrap();
    // let mut points = vec![];
    let mut linenum = 0;
    let mut maxx = -1000;
    let mut maxy = -1000;
    let mut minx = 1000;
    let mut miny = 1000;

    // let one_sec = time::Duration::from_millis(1000);

    let mut state = [[0i32; 301]; 301];
    let input = 3628;

    for i in 1..301 {
        for j in 1..301 {
            let rackid = i as i32 + 10;
            state[i][j] = ((rackid * (j as i32) + input) * rackid / 100) % 10 - 5;
        }
    }
    let mut result = -1000;
    let mut ri = 1;
    let mut rj = 1;
    let mut rs = 1;
    let mut ss1 = [[0i32; 301]; 301];
    // let mut ss2 = [[0i32; 301]; 301];
    for size in 1..301 {
        let mut ss2 = [[0i32; 301]; 301];
        for i in 1..(301 - size) {
            for j in 1..(301 - size) {
                let mut h = 0;
                let mut w = 0;
                for m in j..(size+j) {
                    // println!("{}-{}", i+size-1, m);
                    h += state[i+size-1][m];
                }
                for n in i..(size+i-1) {
                    w += state[n][j+size-1];
                }
                ss2[i][j] = ss1[i][j]+h+w;
                // println!("{}-{}-{}-{}", size,i,j,ss2[i][j]);
                if ss2[i][j] > result {
                    result = ss2[i][j];
                    ri = i;
                    rj = j;
                    rs = size;
                }
            }
        }
        ss1 = ss2;
    }
    println!("{}-{}-{}-{}", result, ri, rj, rs);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    X: i32,
    Y: i32,
    Vx: i32,
    Vy: i32,
}
