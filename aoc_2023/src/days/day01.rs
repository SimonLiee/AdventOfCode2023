use std::fs::read_to_string;

const EX_INPUT1 : &str = "./input/day01ex1";
const EX_INPUT2 : &str = "./input/day01ex2";
const INPUT : &str = "./input/day01";

fn get_input(example: bool, part: i32) -> Vec<String>{
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

    let mut num : String;
    let mut first_num: char;
    let mut last_num: char;
    for i in input {
        num = "".to_string();
        first_num = 'a';
        last_num = 'b';
        for j in i.chars() {
            if j.is_numeric() {
                if first_num == 'a' {
                    first_num = j;
                } 
                last_num = j;
            }
        }
        num.push(first_num);
        num.push(last_num);
        total += num.parse::<i32>().unwrap();
    }

    println!("Result: {}", total);
}

const WORD_NUMS: [&str; 18] = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "one", 
    "two", 
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 2);

    let mut total = 0;

    let mut num : String;
    let mut first_num: char;
    let mut last_num: char;
    let mut first_num_pos;
    let mut last_num_pos;
    for i in input {
        num = "".to_string();
        first_num = 'a';
        last_num = 'b';
        first_num_pos = i.len();
        last_num_pos = 0;
        for (k, el) in WORD_NUMS.iter().enumerate() {
            let res = i.find(el);
            if res.is_none() {
                continue;
            }
            if first_num_pos > res.unwrap() || first_num == 'a' {
                if k >= 9 {
                    first_num = WORD_NUMS[k - 9].chars().collect::<Vec<char>>()[0];
                } else {
                    first_num = WORD_NUMS[k].chars().collect::<Vec<char>>()[0];
                }
                first_num_pos = res.unwrap();
            }
            let res = i.rfind(el);
            if res.is_none() {
                continue;
            }
            if last_num_pos < res.unwrap() {
                if k >= 9 {
                    last_num = WORD_NUMS[k - 9].chars().collect::<Vec<char>>()[0];
                } else {
                    last_num = WORD_NUMS[k].chars().collect::<Vec<char>>()[0];
                }
                last_num_pos = res.unwrap();
            }
        }

        if last_num == 'b' {
            last_num = first_num;
        }
        num.push(first_num);
        num.push(last_num);
        dbg!(&num);
        total += num.parse::<i32>().unwrap();
    }

    println!("Result: {}", total);
}