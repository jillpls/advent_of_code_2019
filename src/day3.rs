use crate::Exercise;
use std::fs;
use std::cmp::{min, max};

pub fn run(exercise : Exercise) {
    let input = parse_input();
    let input1 = to_coordinates(input.0);
    let input2 = input.1;
    let mut current_position = (0, 0);
    let mut intersections : Vec<(i32, i32)> = Vec::new();
    let mut min_step = i32::max_value();
    let mut steps_a = 0;
    let mut steps_b = 0;
    for instruction in input2 {
        let dir : char = instruction.chars().next().expect("Something went wrong ...");
        let length = (&instruction[1..]).parse::<i32>().expect("Could not parse integer");
        let move_vec = get_move_vec(length, dir);
        let new_pos = sum(current_position, move_vec);
        steps_a += get_steps(move_vec);
        let mut idx = 0;
        while idx < input1.len() {
            let coord = input1[idx];
            let old_pos1 = if idx == 0 {
                (0,0)
            } else {
                input1[idx-1].get_tuple()
            };

            let new_step = get_steps(subtract(coord.get_tuple(), old_pos1));
            steps_b += new_step;
            if let Some(i) = find_intersections(current_position, new_pos, old_pos1, coord.get_tuple()) {
                if i != (0,0) {
                    let steps_calc = steps_a - get_steps(subtract(new_pos, i)).abs() + steps_b - get_steps(subtract(coord.get_tuple(), i)).abs();
                    if steps_calc < min_step {
                        min_step = steps_calc;
                    }
                    intersections.push(i);
                }
            }
            idx += 1;
        }
        steps_b = 0;
        current_position = new_pos;
    }
    let mut min = i32::max_value();
    for i in intersections {
        let value = i.0.abs() + i.1.abs();
        if value < min {
            min = value;
        }
    }
    match exercise {
        Exercise::One => {
            println!("{}", min);
        }
        Exercise::Two => {
            println!("{}", min_step);
        }
        Exercise::Both => {
            println!("{}", min);
            println!("{}", min_step);
        }
    }
}

fn find_intersections(start_a : (i32, i32), end_a : (i32, i32),
                    start_b : (i32, i32), end_b : (i32, i32)) -> Option<(i32, i32)> {
    let x_1 = start_a.0;
    let x_2 = end_a.0;
    let x_3 = start_b.0;
    let x_4 = end_b.0;
    let y_1 = start_a.1;
    let y_2 = end_a.1;
    let y_3 = start_b.1;
    let y_4 = end_b.1;
    // println!("({}, {}) -> ({}, {}), ({}, {}) -> ({}, {})", x_1, y_1, x_2, y_2, x_3, y_3, x_4, y_4);
    if x_1 == x_2 && y_3 == y_4 {
        let i = (x_1, y_3);
        if i.0 <= max(x_3, x_4) && i.0 >= min(x_3, x_4) && i.1 <= max(y_1, y_2) && i.1 >= min(y_1, y_2) {
            return Some(i);
        }
    } else if x_3 == x_4 && y_1 == y_2 {
        let i = (x_3, y_1);
        if i.0 <= max(x_1, x_2) && i.0 >= min(x_1, x_2) && i.1 <= max(y_3, y_4) && i.1 >= min(y_3, y_4) {
            return Some(i);
        }
    }

//    let p_1 = start_a;
//    let p_2 = end_a;
//    let p_3 = start_b;
//    let p_4 = end_b;

    None
}

fn get_steps(input : (i32, i32)) -> i32 {
    input.0.abs() + input.1.abs()
}

fn parse_input() -> (Vec<String>, Vec<String>) {
    let content = fs::read_to_string("input/input3").expect("Input Failed");
    let lines : Vec<&str> = content.split('\n').collect();
    let instructions1 : Vec<&str> = lines[0].trim().split(',').collect(); 
    let instructions2 : Vec<&str> = lines[1].trim().split(',').collect(); 
    (instructions1.iter().map(|s| s.to_string()).collect(), instructions2.iter().map(|s| s.to_string()).collect())
}

fn get_move_vec(length : i32, dir : char) -> (i32, i32) {
    match dir {
        'R' => { ( length, 0) },
        'L' => { (-length, 0) },
        'U' => { (0, length) },
        'D' => { (0,-length) },
        _ => { (0,0)}
    }
}

fn to_coordinates(input : Vec<String>) -> Vec<Coordinates> {
    let mut coordinates = Vec::new();
    let mut last = (0, 0);
    for instruction in input {
        let dir : char = instruction.chars().next().expect("Something went wrong ...");
        let length = (&instruction[1..]).parse::<i32>().expect("Could not parse integer");
        let move_vec = get_move_vec(length, dir);
        last = sum(last, move_vec);
        coordinates.push(Coordinates::new(last.0, last.1));
    }
    coordinates
}

fn sum(a : (i32, i32), b : (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn subtract(a : (i32, i32), b : (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x : i32,
    y : i32,
}

impl Coordinates {
    fn new(x : i32, y : i32) -> Self {
        Coordinates {
            x, y
        }
    }

    fn get_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    } 
}