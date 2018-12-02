use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet};

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut twos = 0;
    let mut threes = 0;
    let mut inputs = vec![];
    for line in file.lines() {
        let l = line.unwrap();
        inputs.push(l.clone());
        let (two, three) = parse(&l);
        twos += two;
        threes += three;
        if two > 0 || three > 0 {
            println!("{}-{}-{}", l, two, three);
        }
    }
    println!("{}-{}-{}", twos, threes, twos*threes);

    let len = inputs.len();
    for i in (0..len) {
        for j in (i+1..len) {
            if let Some(result) = common(&inputs[i], &inputs[j]) {
                println!("{}-{}-{}", inputs[i], inputs[j], result);
                return;
            }
        }
    }
}

fn common(a: &str, b: &str) -> Option<String> {
    let alen = a.len();
    let blen = b.len();
    if alen != blen {
        return None;
    }
    let mut differ = None;
    let mut count = 0;
    let mut achars = a.chars();
    let mut bchars = b.chars();
    while let Some(ac) = achars.next() {
        count+=1;
        if let Some(bc) = bchars.next() {
            if ac != bc {
                if differ.is_none() {
                    differ = Some(count);
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
    }
    if differ.is_some() {
        let differ = differ.unwrap();
        return Some(format!("{}{}", a.get(0..differ-1).unwrap(), a.get(differ..alen).unwrap()));
    }

    None
}

fn parse(l: &str) -> (i64, i64) {
    let mut m = HashMap::new();
    for c in l.chars() {
        *m.entry(c).or_insert(0) += 1;
    }
    let mut two = 0;
    let mut three = 0;
    for (_, v) in m.iter() {
        if *v == 2 {
            two = 1;
        } else if *v == 3 {
            three = 1;
        }
    }
    (two, three)
}