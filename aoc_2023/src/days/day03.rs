use std::fs::read_to_string;
use std::collections::HashMap;

const EX_INPUT1 : &str = "./input/day03ex1";
const EX_INPUT2 : &str = "./input/day03ex2";
const INPUT : &str = "./input/day03";

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
    let mut total = 0;
    
    let mut parsed_input : Vec<Vec<char>> = Vec::new();
    for i in input {
        parsed_input.push(i.chars().collect::<Vec<char>>())
    }

    for (i, row) in parsed_input.iter().enumerate() {
        let mut num = "".to_string();
        let mut touch = false;
        for (j, col) in row.iter().enumerate() {
            if col.is_numeric() {
                num.push(*col);
                if check_touch(j, i, &parsed_input) {
                    touch = true;
                }
            } 
            if !col.is_numeric() || j == row.len() - 1 {
                if touch == true {
                    if num != ""{
                        total += num.parse::<u32>().unwrap();
                    }
                }
                num = "".to_string();
                touch = false;
            }
            
        }
    }

    println!("Result: {}", total);
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 2);
    let mut total = 0;
    
    let mut parsed_input : Vec<Vec<char>> = Vec::new();
    for i in input {
        parsed_input.push(i.chars().collect::<Vec<char>>())
    }

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (i, row) in parsed_input.iter().enumerate() {
        let mut num = "".to_string();
        let mut touch = false;
        let mut touch_pos = (0,0);
        for (j, col) in row.iter().enumerate() {
            if col.is_numeric() {
                num.push(*col);
                let res = check_touch2(j, i, &parsed_input);
                if res.is_ok()  {
                    touch = true;
                    touch_pos = res.unwrap();
                }
            } 
            if !col.is_numeric() || j == row.len() - 1 {
                if touch == true {
                    let gearnums : &mut Vec<u32>;
                    if gears.contains_key(&touch_pos) {
                        gearnums = gears.get_mut(&touch_pos).unwrap();
                    } else {
                        gears.insert(touch_pos, Vec::new());
                        gearnums = gears.get_mut(&touch_pos).unwrap();
                    }
                    gearnums.push(num.parse().unwrap())
                }
                num = "".to_string();
                touch = false;
            }
        }
    }
    for (_gear, nums) in gears {
        if nums.len() == 2 {
            total += nums[0] * nums[1];
        }
    }
    
    println!("Result: {}", total);
}

fn check_touch(x: usize, y: usize, input: &Vec<Vec<char>>) -> bool {
    if y < input.len() - 1 { 
        if input[y + 1][x] != '.' && !input[y + 1][x].is_numeric() {
            return true;
        }
        if x > 0 {
            if input[y + 1][x - 1] != '.' && !input[y + 1][x - 1].is_numeric() {
                return true;
            }
        }
        if x < input[y + 1].len() - 1 { 
            if input[y + 1][x + 1] != '.' && !input[y + 1][x + 1].is_numeric() {
                
                return true;
            }
        }
    }
    if y > 0 {
        if input[y - 1][x] != '.' && !input[y - 1][x].is_numeric() {
            return true;
        }
        if x > 0 {
            if input[y - 1][x - 1] != '.' && !input[y - 1][x - 1].is_numeric() {
                return true;
            }
        }
        if x < input[y - 1].len() - 1 { 
            if input[y - 1][x + 1] != '.' && !input[y - 1][x + 1].is_numeric() {
                return true;
            }
        }
    }
    if x < input[y].len() - 1 { 
        if input[y][x + 1] != '.' && !input[y][x + 1].is_numeric() {
            return true;
        }
        if x > 0{
            if input[y][x - 1] != '.' && !input[y][x - 1].is_numeric() {
                return true;
            }
        }
    }
    return false;
}

fn check_touch2(x: usize, y: usize, input: &Vec<Vec<char>>) -> Result<(usize,usize), String> {
    if y < input.len() - 1 { 
        if input[y + 1][x] != '.' && !input[y + 1][x].is_numeric() {
            return Ok((x, y + 1));
        }
        if x > 0 {
            if input[y + 1][x - 1] != '.' && !input[y + 1][x - 1].is_numeric() {
                return Ok((x - 1, y + 1));
            }
        }
        if x < input[y + 1].len() - 1 { 
            if input[y + 1][x + 1] != '.' && !input[y + 1][x + 1].is_numeric() {
                return Ok((x + 1, y + 1));
            }
        }
    }
    if y > 0 {
        if input[y - 1][x] != '.' && !input[y - 1][x].is_numeric() {
            return Ok((x, y - 1));
        }
        if x > 0 {
            if input[y - 1][x - 1] != '.' && !input[y - 1][x - 1].is_numeric() {
                return Ok((x - 1, y - 1));
            }
        }
        if x < input[y - 1].len() - 1 { 
            if input[y - 1][x + 1] != '.' && !input[y - 1][x + 1].is_numeric() {
                return Ok((x + 1, y - 1));
            }
        }
    }
    if x < input[y].len() - 1 { 
        if input[y][x + 1] != '.' && !input[y][x + 1].is_numeric() {
            return Ok((x + 1, y));
        }
        if x > 0{
            if input[y][x - 1] != '.' && !input[y][x - 1].is_numeric() {
                return Ok((x - 1, y));
            }
        }
    }

    return Err("No touch".to_string());
}