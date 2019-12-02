mod day1;
mod day2;
use std::env;

const CURRENT_DAY : u32 = 2;

fn main() { 
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

    match day {
        1 => day1::run(exercise),
        2 => day2::run(exercise),
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