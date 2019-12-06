/*
 *   Copyright (c) 2019 Jill Enke <jill.enke@gmail.com>
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

use crate::Exercise;
use std::collections::{HashMap, HashSet};
use std::cell::{RefCell};

pub fn run(exercise : Exercise) {
    let orbits = get_input();
    let mut orbit_map = HashMap::<String, RefCell<(String, i64, HashSet<String>)>>::new();
    for o in orbits {
        if orbit_map.get(&o.0).is_none() {
            orbit_map.insert(o.0.clone(), RefCell::from((o.0.clone(), 0, HashSet::<String>::new())));
        }
        orbit_map.insert(o.1, RefCell::from((o.0.clone(), -1, HashSet::<String>::new())));
    }
    let mut count = 0;
    for k in orbit_map.keys() {
        calc_orbit_count(&k, &orbit_map, None);
    }
    let mut count = 0;
    for v in orbit_map.values() {
        let v = v.borrow().1;
        count += v;
    }
    match exercise {
        Exercise::One => {
            println!("{}", count);
        }
        Exercise::Two => {
            find_path(&orbit_map, "YOU", "SAN");
        }
        Exercise::Both => {
            println!("{}", count);
            find_path(&orbit_map, "YOU", "SAN");
        }
    }
}

fn find_path(map : &HashMap<String, RefCell<(String, i64, HashSet<String>)>>, a : &str, b : &str) {
    let mut forks_a = Vec::<String>::new(); 
    let mut current_branch : String = a.to_string();
    let mut count = 0;
    while let Some(v) = map.get(&current_branch) {
        count += 1;
        let v = v.borrow();
        // println!("{:?}", v);
        if v.2.len() >= 2 {
            forks_a.push(v.0.clone());
        }
        if current_branch == v.0 {
            current_branch = String::from(")");
        } else {
            current_branch = v.0.clone();
        }
    }
    let mut forks_b = Vec::<String>::new(); 
    let mut current_branch : String = b.to_string();
    while let Some(v) = map.get(&current_branch) {
        let v = v.borrow();
        if v.2.len() >= 2 {
            forks_b.push(v.0.clone());
        }
        if current_branch == v.0 {
            current_branch = String::from(")");
        } else {
            current_branch = v.0.clone();
        }
    }
    let mut i = 1;
    while forks_a[forks_a.len() - i] == forks_b[forks_b.len() -i] {
        i += 1;
        if i > forks_a.len() || i > forks_b.len() { break; }
    }
    let common_fork = forks_a[forks_a.len() - i + 1].clone();
    let to_fork_a = find_length_to(map, a, &common_fork);
    let to_fork_b = find_length_to(map, b, &common_fork);
    println!("{}+{}={}", to_fork_a, to_fork_b, to_fork_a + to_fork_b);
}

fn find_length_to(map : &HashMap<String, RefCell<(String, i64, HashSet<String>)>>, from : &str, to : &str) -> u32 {
    let mut current_node = from.to_string();
    let mut count : i32 = -1;
    while current_node != to.to_string() {
        current_node = map[&current_node].borrow().0.clone();
        count += 1;
    }
    (count-1) as u32
}

fn calc_orbit_count(key : &str, map : &HashMap<String, RefCell<(String, i64, HashSet<String>)>>, called_from : Option<&str>) -> i64 {
    if !(map.contains_key(key)) {
        panic!("WHAT THE FUCK");
    }
    let mut value = map.get(key).expect("oof").borrow_mut();
    if let Some(c) = called_from {
        value.2.insert(c.to_string());
    }
    if value.1 == -1 {
        value.1 = calc_orbit_count(&value.0, map, Some(key)) + 1;
    }
    value.1
}

fn get_input() -> Vec<(String, String)> {
    let input_str = std::fs::read_to_string("input/input6").expect("Input failed.");
    let orbits = input_str.trim().split('\n');
    let mut output = Vec::new();
    for o in orbits {
        let split : Vec<&str> = o.trim().split(")").collect();
        output.push((split[0].to_string(), split[1].to_string()));
    } 
    output
}