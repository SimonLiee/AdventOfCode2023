use std::fs::read_to_string;

const EX_INPUT1 : &str = "./input/day09ex1";
const EX_INPUT2 : &str = "./input/day09ex2";
const INPUT : &str = "./input/day09";

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
        
    let sqns : Vec<Vec<i32>> = input.iter().map(|x|
        x.split_whitespace().map(|y|
            y.parse().unwrap()
        ).collect()
    ).collect();

    let mut result = 0;
    let mut history : Vec<Vec<i32>>;
    for s in sqns {
        history = Vec::new();
        history.push(s);

        while !history.last().unwrap().iter().all(|&x| x == 0) {
            let mut tmp = Vec::new();
            for i in 0..history.last().unwrap().len() {
                if i >= history.last().unwrap().len() - 1 {
                    break;
                }
                tmp.push(history.last().unwrap()[i+1] - history.last().unwrap()[i]);
            }

            history.push(tmp);
        }

        for i in (0..history.len()).rev() {
            if i > 0 {
                let tmp = history[i-1].last().unwrap() + history[i].last().unwrap();
                history[i-1].push(tmp);
            }
        }
        
        result += history[0].last().unwrap();
    }

    println!("Result: {}", result);
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 2);
        
    let sqns : Vec<Vec<i32>> = input.iter().map(|x|
        x.split_whitespace().map(|y|
            y.parse().unwrap()
        ).collect()
    ).collect();

    let mut result = 0;
    let mut history : Vec<Vec<i32>>;
    for s in sqns {
        history = Vec::new();
        history.push(s);

        while !history.last().unwrap().iter().all(|&x| x == 0) {
            let mut tmp = Vec::new();
            for i in 0..history.last().unwrap().len() {
                if i >= history.last().unwrap().len() - 1 {
                    break;
                }
                tmp.push(history.last().unwrap()[i+1] - history.last().unwrap()[i]);
            }

            history.push(tmp);
        }

        for i in (0..history.len()).rev() {
            if i > 0 {
                let tmp = history[i-1][0] - history[i][0];
                history[i-1].insert(0, tmp);
            }
        }
        
        result += history[0][0];
    }

    println!("Result: {}", result);
}