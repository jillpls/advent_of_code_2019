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
    for i in 0..6 {
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