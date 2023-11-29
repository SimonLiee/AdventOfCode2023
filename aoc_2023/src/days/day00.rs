use std::fs::read_to_string;

const EX_INPUT : &str = "./input/day00ex";
const INPUT : &str = "./input/day00";

fn get_input(example: bool) -> Vec<String>{
    let input : &str;
    if example {
        println!("Running with example input.");
        input = EX_INPUT;
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
    let input : Vec<String> = get_input(example);
        
    println!("Input: {:?}", input);

    println!("Result: {}", 0);
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example);

    println!("Input: {:?}", input);

    println!("Result: {}", 1);
}