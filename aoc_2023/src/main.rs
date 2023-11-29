use std::env;

pub mod days;
use days::DAYS;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut example = false;

    // Whether to run example input or full input
    // Defaults to full input
    if args.len() > 1 {
        if args[1] == "1" {
            example = true;
        } else if args[1] == "0" {
            example = false;
        } else {
            println!("Unexpected input. \nSet first argument to 1 for example input or 0 for normal input.");
            return;
        }
    }

    // Run all parts parts if no parameters are provided
    if args.len() <= 2 {
        for i in 0..DAYS.len() {
            println!("----------- Day {} -----------", i);
            println!("Part 1:");
            DAYS[i][0](example);
            println!("\nPart 2:");
            DAYS[i][1](example);
            println!();
        }
        return;
    } 

    let day  = args[2].parse::<usize>().unwrap(); // TODO: Check result
    let mut part = 0;

    if args.len() > 3 {
        part = args[3].parse::<usize>().unwrap(); // TODO: Check result
    }

    // If day is specified run that day
    if day <= 25 {
        // If part is specified only run that part, otherwise run both
        if part == 0 {
            println!("----------- Day {} -----------", day);
            println!("Part 1:");
            DAYS[day][0](example);
            println!("\nPart 2:");
            DAYS[day][1](example);
        } else {
            println!("----------- Day {} -----------", day);
            println!("Part {}:", part);
            DAYS[day][part - 1](example);
        }
    }
}
