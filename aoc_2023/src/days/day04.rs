use std::fs::read_to_string;

const EX_INPUT1 : &str = "./input/day04ex1";
const EX_INPUT2 : &str = "./input/day04ex2";
const INPUT : &str = "./input/day04";

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

fn parse_input(input : Vec<String>) -> Vec<(Vec<u32>, Vec<u32>)>{
    let mut ret: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
    for i in input {
        let mut card : (Vec<u32>, Vec<u32>)= (Vec::new(), Vec::new());

        let game = i.split(':').collect::<Vec<&str>>()[1];
        
        let winning_nums = game.split('|').collect::<Vec<&str>>()[0];
        let your_nums = game.split('|').collect::<Vec<&str>>()[1];
        

        for j in winning_nums.split_whitespace() {
            card.0.push(j.parse().unwrap());
        }
        for j in your_nums.split_whitespace() {
            card.1.push(j.parse().unwrap());
        }

        ret.push(card);
    }

    return ret; 
}

pub fn part1(example : bool) {
    let input : Vec<String> = get_input(example, 1);
        
    let cards = parse_input(input);
    let mut total = 0;
    for card in cards {
        let mut score = 0;
        for winning_num in card.0.iter() {
            for your_num in card.1.iter() {
                if *winning_num == *your_num {
                    if score == 0 {
                        score = 1
                    } else {
                        score = score*2
                    }
                }
            }
        }
        total += score;
    }

    println!("Result: {}", total);
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 2);
        
    let cards = parse_input(input);
    let mut total = 0;
    let mut card_count = vec![1;cards.len()];


    for (i, card) in cards.iter().enumerate() {        
        let mut score = 0;
        for winning_num in card.0.iter() {
            for your_num in card.1.iter() {
                if *winning_num == *your_num {
                    score += 1;
                }
            }
        }
        for _j in 0..card_count[i] {
            for j in 0..score {
                if i + 1 + j < card_count.len(){
                    card_count[i + 1 + j] += 1;
                }
            }
        }
    }
    
    for i in card_count {
        total += i;
    }
    println!("Result: {}", total);
}