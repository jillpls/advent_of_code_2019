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
    match exercise {
        Exercise::One => process(),
        Exercise::Two=> process2(),
        Exercise::Both => {
            process();
            process2();
        }
    }
}

fn process2() {
    let result = find_codes(19_690_720);
    println!("{:?}", result);
    println!("{}", result.0 * 100 + result.1);
}

fn find_codes(result : i64) -> (i64, i64) {
    let values_init = get_values("input/input2");
    for noun in 0..100 { 
            for verb in 0..100 {
            let mut values = values_init.clone();
            values[1] = noun;
            values[2] = verb;
            update_codes(&mut values);
            if values[0] == result {
                return (noun, verb);
            }
        }
    }
    (0, 1001)
}

fn process() {
    let mut values = get_values("input/input2");
    values[1] = 12;
    values[2] = 2;
    update_codes(&mut values);
    println!("{}", values[0]);
}

pub fn get_values(input_file : &str) -> Vec<i64> {
    let input = fs::read_to_string(input_file).expect("Input failed.");
    let splits : Vec<&str> = input.split(',').collect();
    let mut values : Vec<i64> = Vec::new();
    for s in splits {
        values.push(s.trim().parse::<i64>().expect("Parsing to int failed"));
    }
    values
}

fn update_codes(values : &mut Vec<i64>) {
    let mut idx = 0;
    loop {
        match values[idx] {
            1 | 2 => {
                let idx1 = values[idx + 1] as usize;
                let idx2 = values[idx + 2] as usize;
                let result_idx = values[idx + 3] as usize;
                if values[idx] == 1 {
                    values[result_idx] = values[idx1] + values[idx2];
                } else {
                    values[result_idx] = values[idx1] * values[idx2];
                }
                idx += 4;
            }
            99 => {
                break;
            }
            _ => { panic!("Unexpected value."); }
        }
    }
}