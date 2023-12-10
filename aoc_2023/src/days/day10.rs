use std::fs::read_to_string;
use std::collections::HashMap;
use std::usize;

const EX_INPUT1 : &str = "./input/day10ex1";
const EX_INPUT2 : &str = "./input/day10ex2";
const INPUT : &str = "./input/day10";

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
    let input : Vec<String> = get_input(example, 2);
        
    let pipes : Vec<Vec<char>> = input.iter()
        .map(|x| x.chars().collect())
        .collect();

    let mut start_pos = (0, 0);
    for i in 0..pipes.len() {
        let tmp = pipes[i].iter().position(|&x| x == 'S');
        if tmp.is_some() {
            start_pos = (i, tmp.unwrap());
        }
    }


    let pipe_types : HashMap<char, (usize, usize, usize, usize)> = HashMap::from([
        ('L', (1, 1, 0, 0)), // North East South West
        ('F', (0, 1, 1, 0)),
        ('|', (1, 0, 1, 0)),
        ('-', (0, 1, 0, 1)),
        ('J', (1, 0, 0, 1)),
        ('7', (0, 0, 1, 1)),
    ]);

    let mut pos1 = start_pos;
    let mut pos2 = start_pos;
    let mut last_pos1: (usize, usize) = start_pos;
    let mut last_pos2 = start_pos;
    let mut dir1 = (1, 0, 0, 0);
    let mut dir2 = (1, 0, 0, 0);
    let dirs = [(1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1)];

    for dir in dirs {
        let pos = get_pos(start_pos, dir);
        if pipe_types.contains_key(&pipes[pos.0][pos.1]) {
            let pipe_type = *pipe_types.get(&pipes[pos.0][pos.1]).unwrap();
            if check_connects(pipe_type, dir){
                if pos1 == start_pos {
                    pos1 = pos;
                    dir1 = new_dir(pipe_type, dir);
                } else {
                    pos2 = pos;
                    dir2 = new_dir(pipe_type, dir);
                }
                
            }
        }
    }

    let mut count = 1;
    loop{
        let pos = get_pos(pos1, dir1);
        if pos == last_pos1 || pos == pos1 {
            continue;
        }
        if pos.0 >= pipes.len() || pos.1 >= pipes[0].len() {
            continue;
        }
        
        if pipe_types.contains_key(&pipes[pos.0][pos.1]) {
            let pipe_type = *pipe_types.get(&pipes[pos.0][pos.1]).unwrap();
            if check_connects( pipe_type, dir1){
                last_pos1 = pos1;
                pos1 = pos;
                count += 1;

                dir1 = new_dir(pipe_type, dir1);
            }
        }
        if pos1 == pos2 {
            count -= 1;
            break;
        }


        let pos = get_pos(pos2, dir2);
        if pos == last_pos2 || pos == pos2 {
            continue;
        }
        if pos.0 >= pipes.len() || pos.1 >= pipes[0].len() {
            continue;
        }

        if pipe_types.contains_key(&pipes[pos.0][pos.1]) {
            let pipe_type = *pipe_types.get(&pipes[pos.0][pos.1]).unwrap();
            if check_connects( pipe_type, dir2){
                last_pos2 = pos2;
                pos2 = pos;
                dir2 = new_dir(pipe_type, dir2);
            }
        }
        
        if pos1 == pos2 {
            break;
        }
    }

    dbg!(pos1);
    dbg!(pos2);

    println!("Result: {:?}", count);
}

fn get_pos(pos: (usize, usize), dir : (usize, usize, usize, usize)) -> (usize, usize) { 
    if dir.0 == 1 && pos.0 > 0 {
        return (pos.0 - 1, pos.1);
    } else if dir.1 == 1 {
        return (pos.0, pos.1 + 1);
    } else if dir.2 == 1 {
        return (pos.0 + 1, pos.1);
    } else if pos.1 > 0 {
        return (pos.0, pos.1 - 1);
    }

    return pos;
}

fn new_dir(pipe_type : (usize, usize, usize, usize), dir : (usize, usize, usize, usize)) -> (usize, usize, usize, usize)  {
    let mut ret = pipe_type;

    if dir.0 == 1 {
        ret.2 = 0;
    }
    if dir.1 == 1 {
        ret.3 = 0;
    }
    if dir.2 == 1 {
        ret.0 = 0;
    }
    if dir.3 == 1 {
        ret.1 = 0;
    }

    return ret;
}
fn check_connects(pipe1 : (usize, usize, usize, usize), pipe2 : (usize, usize, usize, usize)) -> bool {
    if pipe1.0 == 1 {
        if pipe2.2 == 1 {
            return true;
        }
    }
    if pipe1.1 == 1 {
        if pipe2.3 == 1 {
            return true;
        }
    }
    if pipe1.2 == 1 {
        if pipe2.0 == 1 {
            return true;
        }
    }
    if pipe1.3 == 1 {
        if pipe2.1 == 1 {
            return true;
        }
    }

    return false;
}

pub fn part2(example : bool) {
    let input : Vec<String> = get_input(example, 2);

    println!("Input: {:?}", input);

    println!("Result: {}", 1);
}