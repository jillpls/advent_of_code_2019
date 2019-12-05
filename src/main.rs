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

mod day1;
pub mod day2;
mod day3;
mod day4;
mod day5;
use std::env;

const CURRENT_DAY : u32 = 5;

fn main() { 
    simple_logger::init().expect("Starting logger failed.");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }
    let day = parse_day(args[1].clone());
    let exercise = if args.len() == 3 {
        let val = args[2].parse::<u32>();
        let val = if let Ok(e) = val {
            e
        } else {
            0
        };
        match val {
            1 => { Exercise::One }
            2 => { Exercise::Two }
            _ => { Exercise::Both } 
        }
    } else {
        Exercise::Both
    };

    println!("Running day {} ...", day);

    match day {
        1 => day1::run(exercise),
        2 => day2::run(exercise),
        3 => day3::run(exercise),
        4 => day4::run(exercise),
        5 => day5::run(exercise),
        _ => usage()
    }
}

fn parse_day(day_str : String) -> u32 {
    let day = day_str.parse::<u32>();
    if let Ok(d) = day {
        if d >= 1 && d <= CURRENT_DAY {
            return d;
        }
    }
    usage();
    0
}

fn usage(){
    println!("Usage:\n ./advent_of_code <day> [1|2 (exercise)]");
    std::process::exit(0);
}

pub enum Exercise {
    One,
    Two,
    Both,
}