use std::fs::read_to_string;

const EX_INPUT1 : &str = "./input/day06ex1";
const EX_INPUT2 : &str = "./input/day06ex2";
const INPUT : &str = "./input/day06";

fn get_input(example: bool, part: u32) -> Vec<String>{
    let input : &str;
    if example {
        println!("Running with example input.");

        if part == 2{
            input = EX_INPUT2;
        } else {
            input = EX_INPUT1;
        }
    } else {
        println!("Running with full input.");
        input = INPUT;
    }
    return read_to_string(input) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn part1(example : bool) {
    let input : Vec<String> = get_input(example, 1);
        
    let times : Vec<u32> = input[0].split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .to_vec()
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let records : Vec<u32> = input[1].split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .to_vec()
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut total = 1;
    for (record, time) in times.iter().enumerate() {
        let mut win = 0;
        for i in 0..*time{
            if (time - i) * i > records[record] {
                win += 1;
            }
        }
        total *= win;
    }

    println!("Result: {}", total);
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 1);
        
    let time : u64 = input[0].split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .to_vec()
        .join("")
        .parse()
        .unwrap();
        
    let record : u64 = input[1].split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .to_vec()
        .join("")
        .parse()
        .unwrap();

    
    let mut total = 0;
    for i in 0..time{
        if (time - i) * i > record {
            total += 1;
        }
    }

    println!("Result: {}", total);
}