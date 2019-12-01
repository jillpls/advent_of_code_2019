use crate::Exercise;
use std::fs;

pub fn run(exercise : Exercise) {
    let input = fs::read_to_string("input/input1").expect("Input failed");
    match exercise {
        Exercise::One => {
            calc(input, false);
        }
        Exercise::Two => {
            calc(input, true);
        }
        _ => {
            calc(input.clone(), false);
            calc(input, true);
        }
    }
}

fn calc(input : String, second : bool) {
    let values : Vec<&str> = input.split('\n').collect();
    let mut sum = 0;
    for v in values {
        let v_int : u32 = v.trim().parse().unwrap();
        let mut fuel = get_fuel(v_int);
        sum += fuel;
        if second {
            while fuel > 0 {
                fuel = get_fuel(fuel);
                sum += fuel;
            }
        }
    }
    println!("{}", sum);
}

fn get_fuel(mass : u32) -> u32 {
    let result = (mass as f32 / 3.0).floor() as i32 - 2;
    if result < 0 {
        0
    } else {
        result as u32
    }
}