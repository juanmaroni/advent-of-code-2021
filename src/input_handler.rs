// Functions to parse problem inputs

use std::fs::File;
use std::io::{BufRead, BufReader};

fn build_reader(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("Something went wrong opening the file");
    let reader = BufReader::new(file);

    reader
}

pub fn parse_nums(filename: &str) -> Vec<u32> {
    let mut lines = Vec::new();

    for line in build_reader(filename).lines() {
        let line = line.unwrap().parse::<u32>().expect("Not a number");
        lines.push(line);
    }

    lines
}

pub fn parse_commands(filename: &str) -> Vec<(String, u32)> {
    let mut commands = Vec::new();

    for line in build_reader(filename).lines() {
        let command = line.unwrap();
        let args: Vec<&str> = command.split_whitespace().collect();
        commands.push((args[0].to_string(), args[1].parse::<u32>().unwrap()));
    }

    commands
}
