use std::{fs::read_to_string, collections::HashMap};

const EX_INPUT1 : &str = "./input/day12ex1";
const EX_INPUT2 : &str = "./input/day12ex2";
const INPUT : &str = "./input/day12";

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

fn parse_input(input : Vec<String>) -> Vec<(Vec<char>, Vec<u32>)> {
    let ret : Vec<(Vec<char>, Vec<u32>)>;

    ret = input.iter().map( |x| (
                x.split_whitespace()
                .collect::<Vec<&str>>()[0]
                .chars()
                .collect::<Vec<char>>(), 

                x.split_whitespace()
                .collect::<Vec<&str>>()[1]
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse().unwrap())
                .collect(),
        )).collect();

    return ret;
}

pub fn part1(example : bool) {
    let input : Vec<String> = get_input(example, 1);
    
    let springs = parse_input(input);

    let mut total = 0;
    let mut count;
    for i in springs {
        count = check_all((i.0, i.1));
        total += count;
    }


    println!("Result: {}", total);
}

fn check_all(spring : (Vec<char>, Vec<u32>)) -> usize {
    let mut found = false;
    let mut count = 0;

    for i in 0..spring.0.len() {
        if spring.0[i] == '?' {
            let mut spring1 = spring.0.clone();
            let mut spring2 = spring.0.clone();
            spring1[i] = '.';
            spring2[i] = '#';
            count += check_all((spring1, spring.1.clone()));
            count += check_all((spring2, spring.1.clone()));
            found = true;
            break;
        }
    }
    if !found {
        if check_valid(spring.clone()) {
            count += 1;
        } 
    }
    return count;
}

fn check_valid(spring : (Vec<char>, Vec<u32>) ) -> bool {

    let mut started = false;
    let mut start = 0;

    let mut group: usize = 0;

    for i in 0..spring.0.len() {
        if spring.0[i] == '#' && !started{
            started = true;
            start = i;
        } else if spring.0[i] != '#' && started {
            started = false;
            if group >= spring.1.len() {
                return false;
            }
            if i - start != spring.1[group].try_into().unwrap() {
                return false;
            } else {
                group += 1;
            }
        } 
    }
    if started {
        if group >= spring.1.len() {
            return false;
        }
        if spring.0.len() - start != spring.1[group].try_into().unwrap() {
            return false;
        } else {
            group += 1;
        }
    } 

    if group != spring.1.len() {
        return false;
    }

    return true;
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 1);
    
    let springs = parse_input(input);

    let mut total = 0;
    let mut count;
    for i in springs {
        let mut memo = HashMap::<(Vec<char>, Vec<u32>), usize>::new();
        count = check_all2((i.0, i.1), &mut memo);
        dbg!(count);
        total += count;
    }

    println!("Result: {}", total);
}

fn check_all2(mut spring : (Vec<char>, Vec<u32>), memo : &mut HashMap<(Vec<char>, Vec<u32>), usize>) -> usize {
    println!("{:?}", spring);
    if memo.contains_key(&spring) {
        return *memo.get(&spring).unwrap();
    }

    if spring.0.len() == 0 && spring.1.len() == 0 {
        return 1;
    }

    let mut count = 0;

    let mut start = 0;
    let mut started = false;
    
    for i in 0..spring.0.len() {
        if spring.0[i] == '#' && !started{
            started = true;
            start = i;
        } else if spring.0[i] != '.' && started {
            if spring.1.len() <= 0 {
                return 0;
            }
            if i - start != spring.1[0].try_into().unwrap() {
                return 0;
            } else {
                spring.0.drain(0..i);
                spring.1.remove(0);
                
                let spring1 = spring.clone();

                count = check_all(spring1);
                memo.insert(spring, count);
                return count;
            }
        } else if spring.0[i] == '?' {
            let mut spring1 = spring.0.clone();
            let mut spring2 = spring.0.clone();
            spring1[i] = '.';
            spring2[i] = '#';
            count += check_all2((spring1, spring.1.clone()), memo);
            count += check_all2((spring2, spring.1.clone()), memo);
            return count;
        }
    }

    if spring.1.len() > 0 {
        return 0;
    }

    memo.insert(spring, 1);
    return 1;
}