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
    let mut dir1 = (1, 0, 0, 0);
    let dirs = [(1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1)];

    for dir in dirs {
        let pos = get_pos(start_pos, dir);
        if pipe_types.contains_key(&pipes[pos.0][pos.1]) {
            let pipe_type = *pipe_types.get(&pipes[pos.0][pos.1]).unwrap();
            if check_connects(pipe_type, dir){
                if pos1 == start_pos {
                    pos1 = pos;
                    dbg!(dir);
                    dir1 = new_dir(pipe_type, dir);
                    dbg!(dir1);
                } 
            }
        }
    }

    let mut count = 1;
    loop{
        let pos = get_pos(pos1, dir1);
        dbg!(pos1);
        if pipes[pos.0][pos.1] == 'S' {
            break;
        }

        if pipe_types.contains_key(&pipes[pos.0][pos.1]) {
            let pipe_type = *pipe_types.get(&pipes[pos.0][pos.1]).unwrap();
            if check_connects( pipe_type, dir1){
                pos1 = pos;
                count += 1;

                dir1 = new_dir(pipe_type, dir1);
            }
        }
    }
 
    println!("Result: {:?}", count / 2 + 1);
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
    let mut dir1 = (1, 0, 0, 0);
    let dirs = [(1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1)];
    
    let mut loopmap = vec![vec!['O'; pipes[0].len()] ;pipes.len()];

    for dir in dirs {
        let pos = get_pos(start_pos, dir);
        if pipe_types.contains_key(&pipes[pos.0][pos.1]) {
            let pipe_type = *pipe_types.get(&pipes[pos.0][pos.1]).unwrap();
            if check_connects(pipe_type, dir){
                if pos1 == start_pos {
                    pos1 = pos;
                    dir1 = new_dir(pipe_type, dir);
                    break;
                } 
            }
        }
    }

    loopmap[pos1.0][pos1.1] = pipes[pos1.0][pos1.1];

    loop{
        let pos = get_pos(pos1, dir1);
        
        loopmap[pos1.0][pos1.1] = pipes[pos1.0][pos1.1];

        if pipes[pos.0][pos.1] == 'S' {
            loopmap[pos.0][pos.1] = 'L';
            break;
        }

        if pipe_types.contains_key(&pipes[pos.0][pos.1]) {
            let pipe_type = *pipe_types.get(&pipes[pos.0][pos.1]).unwrap();
            if check_connects( pipe_type, dir1){
                pos1 = pos;
                dir1 = new_dir(pipe_type, dir1);
            }
        }
    }

    let start_list = ['F', 'L'];
    let stop_list = ['7', 'J'];

    let mut new: Vec<Vec<char>> = vec![Vec::new() ;pipes.len()];
    for i in 0..loopmap.len() {
        for j in 0..loopmap[i].len() {
            new[i].push(loopmap[i][j]);
            if start_list.contains(&loopmap[i][j]) {
                new[i].push('-');
            } else if stop_list.contains(&loopmap[i][j]) {
                new[i].push('o');
            } else if loopmap[i][j] == '-' {
                new[i].push('-');
            } else {
                new[i].push('o');
            }
        }    
    }

    loopmap = new;
    for i in 0..loopmap.len() {
        loopmap.insert(i*2, Vec::new());
    }
    loopmap.remove(0);
    loopmap.push(Vec::new());

    let start_list = ['F', '7'];
    let stop_list = ['L', 'J'];
    for j in 0..loopmap[0].len(){
        for i in 0..loopmap.len()/2 {
            if start_list.contains(&loopmap[i * 2][j]) {
                loopmap[i*2+1].push('|');
            } else if stop_list.contains(&loopmap[i * 2][j]) {
                loopmap[i*2+1].push('o');
            } else if loopmap[i * 2][j] == '|' {
                loopmap[i*2+1].push('|');
            } else {
                loopmap[i*2+1].push('o');
            }
        }
    }

    let mut run = true;
    while run {
        run = false;
        for i in 0..loopmap.len() {
            for j in 0..loopmap[i].len() {
                if loopmap[i][j] != 'o' && loopmap[i][j] != 'O' {
                    continue;
                }
                if i == 0 || j == 0 || i == loopmap.len() - 1 || j == loopmap[0].len() - 1 {
                    loopmap[i][j] = 'X';
                    run = true;
                    continue;
                }

                if loopmap[i+1][j] == 'X' {
                    loopmap[i][j] = 'X';
                    run = true;
                } else if loopmap[i][j-1] == 'X' {
                    loopmap[i][j] = 'X';
                    run = true;
                } else if loopmap[i-1][j] == 'X' {
                    loopmap[i][j] = 'X';
                    run = true;
                } else if loopmap[i][j+1] == 'X' {
                    loopmap[i][j] = 'X';
                    run = true;
                }
            }
        }
    }

    let mut count = 0;
    for i in &loopmap {
        for j in i {
            if *j == 'O'{
                count += 1;
            }
        }
    }

    println!("Result: {:?}", count);
}