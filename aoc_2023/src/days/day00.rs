use std::fs::read_to_string;

const EX_INPUT1 : &str = "./input/day00ex1";
const EX_INPUT2 : &str = "./input/day00ex2";
const INPUT : &str = "./input/day00";

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
        
    println!("Input: {:?}", input);

    println!("Result: {}", 0);
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 2);

    println!("Input: {:?}", input);

    println!("Result: {}", 1);
}