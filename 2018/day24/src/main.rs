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
    let mut immune_armies = Vec::new();
    let mut infection_armies = Vec::new();
    for line in file.lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            continue;
        }
        
        if l == "Immune System:" {
            immune = true;
        } else if l == "Infection:" {
            immune = false;
        } else {
            let caps = re.captures(&l).unwrap();
            // println!("{:?}", caps);
            let units = caps[1].parse::<i64>().unwrap();
            let hit_points = caps[2].parse::<i64>().unwrap();
            let weak_immune: Vec<&str> = if caps[3].len() > 0 {
                caps[3][1..caps[3].len() - 2].split("; ").collect()
            } else {
                Vec::new()
            };
            let mut weaks = Vec::new();
            let mut immunes = Vec::new();
            for wi in weak_immune {
                if wi.starts_with("weak to ") {
                    weaks = wi
                        .get(8..)
                        .unwrap()
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect();
                } else {
                    immunes = wi
                        .get(10..)
                        .unwrap()
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect();
                }
            }
            let damage = caps[4].parse::<i64>().unwrap();
            let damage_type = caps[5].to_owned();
            let initiative = caps[6].parse::<i64>().unwrap();
            // println!("{:?}-{:?}-{:?}-{:?}-{:?}-{:?}-{:?}", units, hit_points, weaks, immunes, damage, damage_type, initiative);
            let mut army = Army {
                id: (0, 0),
                units: units,
                hit_points: hit_points,
                weaks: weaks,
                immunes: immunes,
                damage: damage,
                damageType: damage_type,
                initiative: initiative,
            };
            if immune {
                immune_count += 1;
                army.id = (0, immune_count);
                immune_armies.push(army);
            } else {
                infection_count += 1;
                army.id = (1, infection_count);
                infection_armies.push(army);
            }
        }
    }

    println!("part1: {}", fight(&immune_armies, &infection_armies, 0).1);

    // find max
    let mut max = 1;
    loop {
        let (win, result) = fight(&immune_armies, &infection_armies, max);
        if win {
            break;
        }
        max *= 2;
    }
    let mut min = max / 2;
    // println!("find boost between {}, {}", min, max);
    loop {
        if max == min || max - min == 1 {
            let (win, result) = fight(&immune_armies, &infection_armies, min);
            if win {
                println!("part2: {}", result);
                break;
            }
            let (win, result) = fight(&immune_armies, &infection_armies, max);
            if win {
                println!("part2: {}", result);
            }
            break;
        }
        let boost = (max + min) / 2;
        let (win, result) = fight(&immune_armies, &infection_armies, boost);
        if win {
            max = boost;
        } else {
            min = boost;
        }
    }
}

fn fight(immune_armies: &Vec<Army>, infection_armies: &Vec<Army>, boost: i64) -> (bool, i64) {
    let mut immune_armies: Vec<Army> = immune_armies
        .iter()
        .map(|e| {
            let mut e = e.clone();
            e.damage += boost;
            e
        })
        .collect();
    let mut infection_armies = infection_armies.clone();

    loop {
        if immune_armies.len() == 0 || infection_armies.len() == 0 {
            let mut left = 0;
            for a in &immune_armies {
                left += a.units;
            }
            for a in &infection_armies {
                left += a.units;
            }
            return (immune_armies.len() > 0, left);
        }
        infection_armies.sort_by(
            |a, b| match (b.effective_power()).cmp(&(a.effective_power())) {
                Ordering::Equal => b.initiative.cmp(&a.initiative),
                ordering => ordering,
            },
        );
        immune_armies.sort_by(
            |a, b| match (b.effective_power()).cmp(&(a.effective_power())) {
                Ordering::Equal => b.initiative.cmp(&a.initiative),
                ordering => ordering,
            },
        );
        let mut army_all = immune_armies.clone();
        army_all.append(&mut infection_armies.clone());
        let mut attackers = Vec::new();
        let mut fighters = HashMap::with_capacity(army_all.len());
        let mut attacked = HashSet::with_capacity(army_all.len());
        // println!("{:?}", army_all);
        for a in &army_all {
            let fromk = a.id.clone();
            let i = a;
            if i.units <= 0 {
                continue;
            }
            let mut d = 0;
            let mut ep = 0;
            let mut it = 0;
            for b in &army_all {
                let tok = b.id.clone();
                if fromk.0 == tok.0 {
                    continue;
                }
                let j = b;
                if j.units <= 0 {
                    continue;
                }
                if j.immunes.contains(&i.damageType) || attacked.contains(&tok) {
                    // println!("jump {:?}, {:?}, {:?}", fromk, tok, attacked);
                    continue;
                }
                let mut tmp = i.effective_power();
                if j.weaks.contains(&i.damageType) {
                    tmp *= 2;
                }
                let find = (d < tmp)
                    || (tmp == d && ep < j.effective_power())
                    || (tmp == d && ep == j.effective_power() && it < j.initiative);
                if find {
                    d = tmp;
                    ep = j.effective_power();
                    it = j.initiative;
                    if let Some(pretov) = fighters.get(&fromk) {
                        attacked.remove(pretov);
                    } else {
                        attackers.push((fromk.0, fromk.1, i.initiative));
                    }
                    fighters.insert(fromk.clone(), tok.clone());
                    attacked.insert(tok.clone());
                }
            }
        }
        attackers.sort_by(|a, b| b.2.cmp(&a.2));

        let mut armies = HashMap::with_capacity(army_all.len());
        for (i, a) in infection_armies.iter().enumerate() {
            armies.insert(a.id.clone(), i);
        }
        for (i, a) in immune_armies.iter().enumerate() {
            armies.insert(a.id.clone(), i);
        }

        // println!("{:?}-{:?}-{:?}", fighters, immune_armies, infection_armies);
        let mut hit = false;
        for to in &attackers {
            let (fromtype, fromid, _) = to.to_owned();
            let fromindex = armies.get(&(fromtype, fromid)).unwrap().to_owned();
            let mut from = if fromtype == 0 {
                immune_armies[fromindex as usize].clone()
            } else {
                infection_armies[fromindex as usize].clone()
            };
            let mut damage = from.effective_power();
            if damage < 0 {
                continue;
            }
            let tok = match fighters.get(&(fromtype, fromid)) {
                Some(k) => k,
                None => continue,
            };
            let toindex = armies.get(&tok).unwrap();
            let mut to = if tok.0 == 0 {
                immune_armies[*toindex as usize].clone()
            } else {
                infection_armies[*toindex as usize].clone()
            };
            if to.weaks.contains(&from.damageType) {
                damage *= 2;
            }
            let kill = damage / to.hit_points;
            if kill > 0 {
                hit = true;
                to.units -= kill;
                if tok.0 == 0 {
                    immune_armies[*toindex as usize] = to;
                } else {
                    infection_armies[*toindex as usize] = to;
                };
            }
        }
        if !hit {
            return (false, -1);
        }

        immune_armies = immune_armies
            .into_iter()
            .filter(|i| i.units > 0)
            .collect::<Vec<_>>();
        infection_armies = infection_armies
            .into_iter()
            .filter(|i| i.units > 0)
            .collect::<Vec<_>>();
        // println!("{:?}-{:?}", immune_armies, infection_armies);
    }
    panic!("xxx")
}

#[derive(Debug, Clone)]
struct Army {
    pub id: (i32, i32),
    pub units: i64,
    pub hit_points: i64,
    pub weaks: Vec<String>,
    pub immunes: Vec<String>,
    pub damage: i64,
    pub damageType: String,
    pub initiative: i64,
}

impl Army {
    pub fn effective_power(&self) -> i64 {
        self.units * self.damage
    }
}
