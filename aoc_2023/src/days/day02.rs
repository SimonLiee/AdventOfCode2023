use std::fs::read_to_string;

const EX_INPUT1 : &str = "./input/day02ex1";
const EX_INPUT2 : &str = "./input/day02ex2";
const INPUT : &str = "./input/day02";

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

fn parse_input(input : Vec<String>) -> Vec<Vec<Vec<(u32, String)>>> {
    let mut ret: Vec<Vec<Vec<(u32, String)>>> = Vec::new();
    for i in input {
        let mut games : Vec<Vec<(u32, String)>> = Vec::new();

        let game = i.split(':').collect::<Vec<&str>>()[1];
        let sets = game.split(";").collect::<Vec<&str>>();

        for j in sets {
            let mut set = Vec::new();
            for k in j.to_string().split(",") {
                let tmp : Vec<&str> = k.split_whitespace().collect();
                set.push((tmp[0].parse().unwrap(), tmp[1].to_string()));
            }

            games.push(set);
        }
        ret.push(games);
    }

    return ret; 
}

pub fn part1(example : bool) {
    let input : Vec<String> = get_input(example, 1);
    
    let games = parse_input(input);
    
    let max_red = 12;
    let max_blue = 14;
    let max_green = 13;
    
    let mut total = 0;

    for (i, game) in games.iter().enumerate() {
        let mut invalid : bool = false;
        for set in game {
            for cube in set {
                if cube.1 == "green" {
                    if cube.0 > max_green {
                        invalid = true
                    }
                } else if cube.1 == "red" {
                    if cube.0 > max_red {
                        invalid = true
                    }
                } else {
                    if cube.0 > max_blue {
                        invalid = true
                    }
                }
            }
        }
        if !invalid {
            total += i+1;
        }
    }

    println!("Result: {}", total);
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 1);
    
    let games = parse_input(input);
    
    let mut total = 0;

    for (_i, game) in games.iter().enumerate() {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for set in game {
            for cube in set {
                if cube.1 == "green" {
                    if cube.0 > max_green {
                        max_green = cube.0;
                    }
                } else if cube.1 == "red" {
                    if cube.0 > max_red {
                        max_red = cube.0;
                    }
                } else {
                    if cube.0 > max_blue {
                        max_blue = cube.0;
                    }
                }
            }
        }
        total += max_blue * max_green * max_red;
    }

    println!("Result: {}", total);
}