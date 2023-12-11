use std::fs::read_to_string;

const EX_INPUT1 : &str = "./input/day11ex1";
const EX_INPUT2 : &str = "./input/day11ex2";
const INPUT : &str = "./input/day11";

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
        
    let mut image : Vec<Vec<char>> = input.iter().map(
            |x| x.chars().collect::<Vec<char>>()
        )
        .collect();

    let mut tmp = Vec::new();
    for i in 0..image.len() {
        tmp.push(image[i].clone());
        if !image[i].contains(&'#') {
            tmp.push(image[i].clone());
        }
    }

    let mut ipos = 0;
    for i in 0..image[0].len() {
        let mut found = false;
        for j in 0..image.len() {
            if image[j][i] == '#' {
                found = true
            }
        }

        if !found {
            for j in 0..tmp.len() {
                tmp[j].insert(ipos, '.');
            }
            ipos += 1;
        }
        ipos += 1;
    }

    image = tmp;

    let mut galaxies = Vec::new();
    for i in 0..image.len() {
        for j in 0..image[i].len() {
            if image[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if galaxies[i].0 > galaxies[j].0 {
                total += galaxies[i].0 - galaxies[j].0;
            } else {
                total += galaxies[j].0 - galaxies[i].0;
            }
            if galaxies[i].1 > galaxies[j].1 {
                total += galaxies[i].1 - galaxies[j].1;
            } else {
                total += galaxies[j].1 - galaxies[i].1;
            }
            
        }
    }

    println!("Result: {:?}", total);
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 1);
        
    let image : Vec<Vec<char>> = input.iter().map(
            |x| x.chars().collect::<Vec<char>>()
        )
        .collect();

    // Get position of galaxies
    let mut galaxies = Vec::new();
    for i in 0..image.len() {
        for j in 0..image[i].len() {
            if image[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }
    let mut new_galaxies = galaxies.clone();

    for i in 0..image.len() {
        if !image[i].contains(&'#') {
            for j in 0..galaxies.len() {
                if galaxies[j].0 > i {
                    new_galaxies[j].0 += 1000000 - 1;
                }
            }
        }
    }
    for i in 0..image[0].len() {
        let mut found = false;
        for j in 0..image.len() {
            if image[j][i] == '#' {
                found = true
            }
        }

        if !found {
            for j in 0..galaxies.len() {
                if galaxies[j].1 > i {
                    new_galaxies[j].1 += 1000000 - 1;
                }
            }
        }
    }

    galaxies = new_galaxies;
    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if galaxies[i].0 > galaxies[j].0 {
                total += galaxies[i].0 - galaxies[j].0;
            } else {
                total += galaxies[j].0 - galaxies[i].0;
            }
            if galaxies[i].1 > galaxies[j].1 {
                total += galaxies[i].1 - galaxies[j].1;
            } else {
                total += galaxies[j].1 - galaxies[i].1;
            }
            
        }
    }
    
    println!("Result: {:?}", total);
}