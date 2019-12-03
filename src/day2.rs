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

fn find_codes(result : u32) -> (u32, u32) {
    let values_init = get_values();
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
    let mut values = get_values();
    values[1] = 12;
    values[2] = 2;
    update_codes(&mut values);
    println!("{}", values[0]);
}

fn get_values() -> Vec<u32> {
    let input = fs::read_to_string("input/input2").expect("Input failed.");
    let splits : Vec<&str> = input.split(',').collect();
    let mut values : Vec<u32> = Vec::new();
    for s in splits {
        values.push(s.trim().parse::<u32>().expect("Parsing to int failed"));
    }
    values
}

fn update_codes(values : &mut Vec<u32>) {
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