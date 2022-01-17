use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut horizontal_position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim = 0;

    for line in reader.lines() {

        let line = line.unwrap();
        let cmd_vec: Vec<&str> = line.split(" ").collect();

        if cmd_vec.len() != 2 {
            panic!("Malformed command line!");
        }

        let command = cmd_vec[0];
        let param = cmd_vec[1].parse::<i32>().unwrap();

        // println!("Command: {}\tDistance: {}", command, param);

        if command == "forward" {

            horizontal_position += param;
            depth += aim*param;

        } else if command == "down" {
            
            aim += param;
            
        } else if command == "up" {

            aim -= param;

        } else {
            panic!("Unrecognised command");
        }

    }
    println!("Horizontral pos: {}\nDepth: {}", horizontal_position, depth);

    println!("Puzzle answer: {}", horizontal_position*depth);
}
