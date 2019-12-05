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
use std::fs;

pub fn run(exercise : Exercise) {
    let content = fs::read_to_string("input/input4").expect("Input Failed");
    let numbers : Vec<&str> = content.split('-').collect();
    let bounds = (numbers[0].to_string().trim().parse::<i64>().expect(""), numbers[1].to_string().trim().parse::<i64>().expect(""));
    let mut current = bounds.0;
    let mut simple_pwds = 0;
    let mut pwds = 0;
    while current <= bounds.1 {
        let increase = is_increasing(current);
        if increase == 0 {
            pwds += 1;
            current += 1;
        } else if increase == -1 {
            simple_pwds +=1;
            current += 1;
        } else {
            current += increase;
        }
    }
    match exercise {
        Exercise::One => {
            println!("{}", pwds + simple_pwds);
        },
        Exercise::Two => {
            println!("{}", pwds);
        },
        Exercise::Both => {
            println!("{}", pwds + simple_pwds);
            println!("{}", pwds);
        }
    }
}

fn is_increasing(number : i64) -> i64 {
    let mut pow = 10;
    let mut last_digit = 10;
    let mut has_duplicate = false;
    let mut potential_duplicate = false;
    let mut lock = false;
    for _ in 0..6 {
        let digit = (number % pow) / (pow / 10);
        if digit > last_digit {
            let rest = number % (pow / 100);
            let increase = (digit - last_digit) * (pow / 100) - rest;
            //println!("({},{})", digit, last_digit);
            //println!("{} + {}, {}", number, increase, rest);
            return increase;
        }
        if last_digit == digit {
            if !lock {
                potential_duplicate = true;
                lock = true;
            } else {
                potential_duplicate = false;
            }
        } else if potential_duplicate {
            has_duplicate = true;
        } else {
            lock = false;
        }
        last_digit = digit;
        pow *= 10;
    }
    if potential_duplicate || has_duplicate {
        0
    } else {
        -1
    }
}