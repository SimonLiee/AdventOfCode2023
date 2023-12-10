use std::{fs::read_to_string, collections::HashMap};

const EX_INPUT1 : &str = "./input/day08ex1";
const EX_INPUT2 : &str = "./input/day08ex2";
const INPUT : &str = "./input/day08";

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

fn parse_input(mut input : Vec<String>) -> (String, HashMap<String, (String, String)>) {
    let dir = input[0].clone();
    input.remove(0);
    input.remove(0);

    let mut nodes : HashMap<String, (String, String)> = HashMap::new();
    for mut node in input {
        node = node.replace("(", "");
        node = node.replace(")", "");
        node = node.replace(",", "");
        let splt : Vec<&str> = node.split_whitespace().collect();

        nodes.insert(splt[0].to_string(), (splt[2].to_string(), splt[3].to_string()));
    }

    return (dir, nodes);
}

pub fn part1(example : bool) {
    let input : Vec<String> = get_input(example, 1);
    let (dir, nodes) = parse_input(input);

    let mut current = "AAA".to_string();
    let mut count: u128 = 0;
    while current != "ZZZ" {
        for i in dir.chars() {
            count += 1;
            if i == 'R' {
                current = nodes.get(&current).unwrap().1.clone();
            } else {
                current = nodes.get(&current).unwrap().0.clone();
            }
            if current == "ZZZ" {
                break;
            }
        }
    }

    println!("Result: {}", count);
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 2);
    let (dir, nodes) = parse_input(input);

    let mut current = Vec::new();

    for i in nodes.keys() {
        if i.ends_with('A') {
            current.push(i.clone());
        }
    }

    let mut counts = vec![0; current.len()];
    for (i, elem) in current.iter().enumerate() {
        let mut curr = elem.clone();
        let mut count = 0;
        while !curr.ends_with("Z"){
            for c in dir.chars() {
                count += 1;
                if c == 'R' {
                    curr = nodes.get(&curr).unwrap().1.clone();
                } else {
                    curr = nodes.get(&curr).unwrap().0.clone();
                }
                if curr.ends_with("Z") {
                    break;
                }
            }
            if curr.ends_with("Z") {
                break;
            }
        }
        counts[i] = count;
    }

    println!("Result: {:?}", findlcm(counts));
    // 12324145107121
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
 
// Returns LCM of array elements
fn findlcm(arr : Vec<u128>) -> u128 {
    // Initialize result
    let mut ans = arr[0];
 
    // ans contains LCM of arr[0], ..arr[i]
    // after i'th iteration,
    for i in 0..arr.len() {
        ans = ((arr[i] * ans)) / (gcd(arr[i], ans));
    }

    return ans;
}