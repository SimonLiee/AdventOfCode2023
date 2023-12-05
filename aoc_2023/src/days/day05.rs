use std::fs::read_to_string;

const EX_INPUT1 : &str = "./input/day05ex1";
const EX_INPUT2 : &str = "./input/day05ex2";
const INPUT : &str = "./input/day05";

fn get_input(example: bool, part: u128) -> Vec<String>{
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

#[derive(Debug)]
struct Rule {
    from : String,
    to : String,
    ranges : Vec<(u128, u128, u128)>
}

#[derive(Debug)]
struct Almanac {
    seeds : Vec<u128>,
    rules : Vec<Rule>
}

fn parse_input(input : Vec<String>) -> Almanac {
    let seeds : Vec<u128> = input[0].split(':')
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|num|num.parse::<u128>().unwrap())
        .collect();
    
    let mut rule : Rule = Rule { from : "".to_string(), to : "".to_string(), ranges : Vec::new()};
    let mut almanac = Almanac {
        seeds : seeds,
        rules : Vec::new(),
    };

    let mut iter = input.iter();
    iter.next();
    while let Some(i) = iter.next() { 
        if i.is_empty() {
            continue;
        }

        let mut split : Vec<&str> = i.split_whitespace().collect();
        if split[1] == "map:" {
            if !rule.from.is_empty(){
                almanac.rules.push(rule);
            }
            split = i.split(|x| x=='-'||x==' ').collect();
            rule = Rule {
                from : split[0].to_string(),
                to : split[2].to_string(),
                ranges : Vec::new(),
            }
        } else {
            rule.ranges.push((
                split[0].parse().unwrap(), 
                split[1].parse().unwrap(), 
                split[2].parse().unwrap()
            ))
        }
    }
    almanac.rules.push(rule);
    return almanac;
}

pub fn part1(example : bool) {
    let input : Vec<String> = get_input(example, 2);
        
    let almanac = parse_input(input);
    let mut results: Vec<u128> = Vec::new();


    for seed in almanac.seeds {
        let mut num = seed;
        for rule in &almanac.rules {
            for range in &rule.ranges {
                if num >= range.1 && num < range.1 + range.2 {
                    num = range.0 + num - range.1;
                    break;
                }
            }
        }
        results.push(num)
    }

    println!("Result: {:?}", results.iter().min().unwrap());
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 1);
        
    let almanac = parse_input(input);
    let mut results: Vec<u128> = Vec::new();
    
    let mut seeds: Vec<(u128, u128)> = Vec::new();
    for i in (0..almanac.seeds.len()).step_by(2) {
        seeds.push((almanac.seeds[i], almanac.seeds[i]+almanac.seeds[i+1]));
    }
    
    let mut next_seeds = Vec::new();
    for rule in almanac.rules {
        dbg!(&seeds);
        dbg!(&rule);
        for range in rule.ranges {
            for i in 0..seeds.len() {
                if !(range.1 + range.2 < seeds[i].0 || range.1 > seeds[i].1) {
                    if range.1 > seeds[i].0 && range.1 + range.2 < seeds[i].1{
                        dbg!("here");
                        next_seeds.push((
                            range.0,
                            range.0 + seeds[i].0 - range.1,
                        ))
                        /*   ----------- seed
                               -------   range */
                    } else if range.1 > seeds[i].0 || range.1 + range.2 < seeds[i].1 {
                        dbg!("here2");
                        seeds.push((seeds[i].0, range.1));
                        seeds.push((range.1 + range.2, seeds[i].1));
                        next_seeds.push((
                            range.0,
                            range.0 + seeds[i].0 - range.1,
                        ))
                        /*       ---       seed
                               -------   range */
                    } else if range.1 > seeds[i].0 {
                        dbg!("here3");
                        seeds.push((seeds[i].0, range.1));
                        next_seeds.push((
                            range.0,
                            range.0 + seeds[i].1 - range.1,
                        ))
                        /* -------       seed
                               -------   range */
                    } else {
                        dbg!("here4");
                        seeds.push((range.1 + range.2, seeds[i].1));
                        next_seeds.push((
                            range.0 + seeds[i].0 - range.1,
                            range.0 + range.2,
                        ))
                        /*      -------       seed
                            -------   range */
                    }
                }
            }
        }
        seeds = next_seeds;
        next_seeds = Vec::new();
    }

    println!("Result: {:?}", &seeds);
}