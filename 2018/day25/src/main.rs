extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::{thread, time};

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    let re = Regex::new(r"^(.*) units each with (.*) hit points (.*?)with an attack that does (.*) (.*) damage at initiative (.*)$").unwrap();
    let mut immune = true;
    let mut immune_count = 0;
    let mut infection_count = 0;
    let mut points = Vec::new();
    let mut linenum = 0;
    for line in file.lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            continue;
        }
        linenum += 1;
        // println!("{}", l);
        let p: Vec<i64> = l.split(",").map(|e| {
            // println!("{}", e);
            e.trim().parse::<i64>().unwrap()
        }).collect();
        points.push((p[0],p[1], p[2],p[3]));
    }
    // println!("{:?}", points);
    
    let mut uf = UF::new(&points);
    for p in &points {
        uf.union(p);
    }
    println!("{}", uf.count());
}

struct UF {
    pub id: HashMap<(i64,i64,i64,i64), i64>,
    pub count: i64,
}

impl UF {
    pub fn new(v: &Vec<(i64,i64,i64,i64)>) -> UF {
        let mut id = HashMap::with_capacity(v.len());
        // for (i,p) in v.iter().enumarate() {
        //     id.insert(p, i);
        // }
        UF {
            id: id,
            count: 0,
        }
    }

    pub fn union(&mut self, newp: &(i64,i64,i64,i64)) {
        let mut find = HashSet::new();
        let mut find_min = -1;
        for (p, v) in &self.id {
            if find.contains(v) {
                continue;
            }
            if self.connected(p, newp) {
                find.insert(v.clone());
                if find_min == -1 || *v<find_min {
                    find_min = *v;
                }
                // break;
            }
        }
        if find.len()>0 {
            // Update all values
            for (_, val) in self.id.iter_mut() {
                if find.contains(val) {
                    *val = find_min;
                }
            }
            self.id.insert(newp.clone(), find_min);
        } else {
            self.count += 1;
            self.id.insert(newp.clone(), self.count);
        }
    }

    pub fn count(&self) -> usize {
        let mut counts = HashSet::new();
        for (_, v) in &self.id {
            counts.insert(v);
        }
        counts.len()
    }

    fn connected(&self, p0: &(i64,i64,i64,i64), p1: &(i64,i64,i64,i64)) -> bool {
        self.abs((p0.0 - p1.0)) + self.abs((p0.1 - p1.1)) + self.abs((p0.2 - p1.2))+ self.abs((p0.3 - p1.3))<=3
    }
    fn abs(&self, i: i64) -> i64 {
        i.abs()
    }

}