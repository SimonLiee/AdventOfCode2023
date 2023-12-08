use std::{fs::read_to_string, cmp::Ordering, collections::HashSet};

const EX_INPUT1 : &str = "./input/day07ex1";
const EX_INPUT2 : &str = "./input/day07ex2";
const INPUT : &str = "./input/day07";

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

fn parse_input(input : Vec<String>) -> Vec<(Vec<u32>, u32)> {
    let mut ret : Vec<(Vec<u32>, u32)> = Vec::new();
    for i in input {
        let hand : Vec<&str>= i.split_whitespace()
            .collect();

        ret.push((
            hand[0].to_string()
                .chars()
                .map(|c|{
                        match c {
                            'A'=> return 14,
                            'K'=> return 13,
                            'Q'=> return 12,
                            'J'=> return 11,
                            'T'=> return 10,
                            _=> return c.to_digit(10).unwrap(),
                    }
                })
                .collect(), 
            hand[1].parse().unwrap()
        ));
    }

    return ret;
}

fn score_hand(hand : &Vec<u32>) -> usize {
    
    let cards : HashSet<u32> = HashSet::from_iter(hand.iter().cloned());
    let mut scores = vec![0;cards.len()];

    for (i, elem1) in cards.iter().enumerate() {
        for (_j, elem2) in hand.iter().enumerate() {
            if elem1 == elem2 {
                scores[i] += 1;
            }
        }
    }
    
    if scores.contains(&5){ // Five of a kind
        return 6;
    } else if scores.contains(&4) { // Four of a kind
        return 5;
    } else if scores.contains(&3) { 
        if scores.contains(&2) { // Full house
            return 4;
        } else {
            return 3; // Three of a kind
        }
    } else if scores.contains(&2) {
        if scores.contains(&3) {  // Full house
            return 4;
        }
        if scores.contains(&2) {  // Two pairs
            for (i, elem) in scores.iter().enumerate() {
                for (j, jelem) in scores.iter().enumerate() {
                    if *elem == 2 && *jelem == 2 && i != j {
                        return 2
                    }
                }
            }
        }
        return 1;
    }

    return 0;
}

fn compare_hands(a : &(Vec<u32>, u32), b : &(Vec<u32>, u32)) -> Ordering {
    let ret : Ordering;

    let ascore = score_hand(&a.0);
    let bscore = score_hand(&b.0);

    if ascore > bscore {
        ret = Ordering::Less;
    } else if ascore < bscore {
        ret = Ordering::Greater;
    } else {
        for i in 0..a.0.len() {
            if a.0[i] < b.0[i] {
                return Ordering::Greater;
            } else if a.0[i] > b.0[i] {
                return Ordering::Less;
            }
            
        }
        ret = Ordering::Equal;
    }

    return ret;
}

pub fn part1(example : bool) {
    let input : Vec<String> = get_input(example, 1);
    let mut hands = parse_input(input);

    hands.sort_by(compare_hands);

    let mut total = 0;
    for (i, elem) in hands.into_iter().rev().enumerate() {
        total += (i + 1) * usize::try_from(elem.1).unwrap();
    }

    println!("Result: {}", total);
}

fn parse_input2(input : Vec<String>) -> Vec<(Vec<u32>, u32)> {
    let mut ret : Vec<(Vec<u32>, u32)> = Vec::new();
    for i in input {
        let hand : Vec<&str>= i.split_whitespace()
            .collect();

        ret.push((
            hand[0].to_string()
                .chars()
                .map(|c|{
                        match c {
                            'A'=> return 13,
                            'K'=> return 12,
                            'Q'=> return 11,
                            'J'=> return 1,
                            'T'=> return 10,
                            _=> return c.to_digit(10).unwrap(),
                    }
                })
                .collect(), 
            hand[1].parse().unwrap()
        ));
    }

    return ret;
}

fn compare_hands2(a : &(Vec<u32>, u32), b : &(Vec<u32>, u32)) -> Ordering {
    let ret : Ordering;
    
    let mut ascore = score_hand(&a.0);
    let mut bscore = score_hand(&b.0);

    for i in &a.0 {
        if *i == 1 {
            for j in 1..14 {
                let score = score_hand(
                    &a.0.iter()
                        .map(|&x| {
                            if x == 1 {
                                return j;
                            } 
                            return x;
                        })
                        .collect()
                );
                if score > ascore {
                    ascore = score;
                }
            }
            break
        }
    }

    for i in &b.0 {
        if *i == 1 {
            for j in 1..14 {
                let score = score_hand(
                    &b.0.iter()
                        .map(|&x| {
                            if x == 1 {
                                return j;
                            } 
                            return x;
                        })
                        .collect()
                );
                if score > bscore {
                    bscore = score;
                }
            }
            break
        }
    }

    if ascore > bscore {
        ret = Ordering::Less;
    } else if ascore < bscore {
        ret = Ordering::Greater;
    } else {
        for i in 0..a.0.len() {
            if a.0[i] < b.0[i] {
                return Ordering::Greater;
            } else if a.0[i] > b.0[i] {
                return Ordering::Less;
            }
            
        }
        ret = Ordering::Equal;
    }

    return ret;
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 1);
    let mut hands = parse_input2(input);

    hands.sort_by(compare_hands2);

    let mut total = 0;
    for (i, elem) in hands.into_iter().rev().enumerate() {
        total += (i + 1) * usize::try_from(elem.1).unwrap();
    }

    println!("Result: {}", total);
}