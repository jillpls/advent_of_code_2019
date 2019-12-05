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

use crate::day2::get_values;
use crate::Exercise;

pub fn run(exercise : Exercise) {
    match exercise {
        Exercise::One => {
            let mut codes = get_values("input/input5");
            process(&mut codes, false);
        }
        Exercise::Two => {
            let mut codes = get_values("input/input5_2");
            process(&mut codes, true);
        }
        _ => {
            panic!("Not supported for today")
        }
    }
}

fn process(codes : &mut Vec<i64>, second : bool) {
    let mut i = 0;
    loop {
        let current_code = *codes.get(i).expect("Code not found.");
        if !second && [5, 6, 7, 8].contains(&(current_code % 100)) {
            panic!("Unexpected code!"); 
        }
        match current_code % 100 {
            1 => {
                let values = get_op_values(current_code, i, codes);
                let result = values.0 + values.1;
                write_result(get_modes(current_code, 3)[3], i+3, result, codes);
                i += 4;
            }
            2 => {
                let values = get_op_values(current_code, i, codes);
                let result = values.0 * values.1;
                write_result(get_modes(current_code, 3)[3], i+3, result, codes);
                i += 4;
            }
            3 => {
                let input = get_input();
                write_result(get_modes(current_code, 1)[1], i+1, input, codes);
                i += 2;
            }
            4 => {
                println!("{}", get_parameter(get_modes(current_code, 1)[1], i+1, codes));
                i += 2;
            }
            5 => {
                i = jump(true, current_code, i, codes);
            }
            6 => {
                i = jump(false, current_code, i, codes);
            }
            7 => {
                ordering(true, current_code, i, codes);
                i += 4;
            }
            8 => {
                ordering(false, current_code, i, codes);
                i += 4;
            }
            99 => {break;}
            _ => { panic!("Unexpected code!") }
        }
    }
}

fn jump(if_what : bool, code : i64, index : usize, codes : &Vec<i64>) -> usize {
    let modes = get_modes(code, 2);
    if (get_parameter(modes[1], index + 1, codes) != 0) == if_what {
        get_parameter(modes[2], index + 2, codes) as usize
    } else {
        index + 3
    }
}

fn ordering(less : bool, code : i64, index : usize, codes : &mut Vec<i64>) {
    let modes = get_modes(code, 3);
    let p1 = get_parameter(modes[1], index + 1, codes);
    let p2 = get_parameter(modes[2], index + 2, codes);
    let result = if less && p1 < p2 {
        1
    } else if !less && p1 == p2 {
        1
    } else {
        0
    };
    write_result(modes[3], index + 3, result, codes)
}

fn get_input() -> i64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not read line");
    input.trim().parse::<i64>().expect("Parsing failed.")
}

fn get_modes(code : i64, count : u32) -> Vec<i64> {
    let mut pow = 1000;
    let mut res = Vec::new();
    res.push(-1);
    for _ in 0..count {
        res.push(
            code % pow / (pow / 10) 
        );
        pow *= 10;
    } 
    res
}

fn get_op_values(code : i64, pos : usize, codes : &Vec<i64>) -> (i64, i64) {
    let modes = get_modes(code, 2);
    let first_parameter = get_parameter(modes[1], pos + 1, codes);
    let second_parameter = get_parameter(modes[2], pos + 2, codes);
    (first_parameter, second_parameter)
}

fn get_parameter(mode : i64, pos : usize, codes : &Vec<i64>) -> i64 {
    match mode {
        0 => { *codes.get(*codes.get(pos).expect("Out of bounds.") as usize).expect("Out of bounds.") }
        1 => { *codes.get(pos).expect("Out of bounds.") }
        _ => { panic!("Invalid Mode!") }
    }
}

fn write_result(mode : i64, pos : usize, result : i64, codes : &mut Vec<i64>) {
    match mode {
        0 => {
            let pos = *codes.get(pos).expect("Out of bounds.") as usize;
            *codes.get_mut(pos).expect("Out of bounds.") = result; }
        1 => { *codes.get_mut(pos).expect("Out of bounds.") = result; }
        _ => { panic!("Invalid Mode!") }
    }
}
