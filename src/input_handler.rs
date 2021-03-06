// Functions to parse problem inputs

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

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

pub fn parse_diagnostic_report(filename: &str) -> Vec<Vec<char>> {
    let mut bits: Vec<Vec<char>> = Vec::new();

    // Build matrix of bits
    for line in build_reader(filename).lines() {
        bits.push(line.unwrap().chars().collect());
    }

    bits
}

pub fn transpose_matrix_chars(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row_len = matrix[0].len();
    let mut transpose_matrix: Vec<Vec<char>> = vec![Vec::with_capacity(matrix.len()); row_len];

    for r in matrix {
        for i in 0..row_len {
            transpose_matrix[i].push(r[i]);
        }
    }

    transpose_matrix
}

pub fn parse_bingo(filename: &str) -> (Vec<u8>, Vec<Vec<Vec<u8>>>) {
    let mut reader = build_reader(filename);

    // Draw numbers
    let mut first_line = String::new();
    reader.read_line(&mut first_line).expect("Could not read line");
    let draw_numbers = first_line.trim().split(',').map(|n| n.parse::<u8>().unwrap()).collect();

    // Boards
    let mut boards: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut board_lines: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        if line.len() == 0 {
            let board_transposed = transpose_matrix_nums_u8(&board_lines);
            boards.push(board_lines);
            boards.push(board_transposed);
            board_lines = Vec::new();
        }
        else {
            let board_line: Vec<u8> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            board_lines.push(board_line);
        }
    }

    (draw_numbers, boards)
}

pub fn transpose_matrix_nums_u8(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{
    let row_len = matrix[0].len();
    let mut transpose_matrix: Vec<Vec<u8>> = vec![Vec::with_capacity(matrix.len()); row_len];

    for r in matrix {
        for i in 0..row_len {
            transpose_matrix[i].push(r[i]);
        }
    }

    transpose_matrix
}

pub fn parse_vents(filename: &str) -> (Vec<(Vec<u16>, Vec<u16>)>, u16) {
    let mut vents: Vec<(Vec<u16>, Vec<u16>)> = Vec::new();
    let mut get_grid_size: Vec<u16> = Vec::new();
    
    for line in build_reader(filename).lines() {
        let l = line.unwrap();
        let mut split = l.split(" -> ");
        let left: Vec<u16> = split.next().unwrap().split(',').map(|n| n.parse::<u16>().unwrap()).collect();
        let right: Vec<u16> = split.next().unwrap().split(',').map(|n| n.parse::<u16>().unwrap()).collect();
        get_grid_size.push(left.clone().into_iter().max().unwrap());
        get_grid_size.push(right.clone().into_iter().max().unwrap());
        let extract_nums: (Vec<u16>, Vec<u16>) = (left, right);
        vents.push(extract_nums);
    }

    (vents, *get_grid_size.iter().max().unwrap() + 1)
}

pub fn parse_line_nums(filename: &str) -> [usize; 9] {
    let f = fs::read_to_string(filename).expect("Error reading file");
    let mut fishes: [usize; 9] = [0; 9];

    for timer in f.trim().split(',').map(|n| n.parse::<usize>().unwrap()) {
        fishes[timer] += 1;
    }

    fishes
}

pub fn parse_positions(filename: &str) -> Vec<u16> {
    let f = fs::read_to_string(filename).expect("Error reading file");
    let positions: Vec<u16> = f.trim().split(',').map(|n| n.parse::<u16>().unwrap()).collect();

    positions
}

pub fn parse_patterns_outputs(filename: &str) -> Vec<([String; 10], [String; 4])>{
    let mut patterns_outputs: Vec<([String; 10], [String; 4])> = Vec::new();

    for line in build_reader(filename).lines() {
        let l = line.unwrap();
        let input_sides: Vec<String> = l.split(" | ").map(|side| side.to_string()).collect();
        let patterns: [String; 10] = input_sides[0].split(" ").map(|segment| segment.to_string()).collect::<Vec<String>>().try_into().unwrap();
        let outputs: [String; 4] = input_sides[1].split(" ").map(|segment| segment.to_string()).collect::<Vec<String>>().try_into().unwrap();
        patterns_outputs.push((patterns, outputs));
    }

    patterns_outputs
}

pub fn parse_heightmap(filename: &str) -> Vec<Vec<i8>> {
    build_reader(filename)
        .lines()
        .into_iter()
        .map(|row| row.unwrap().chars().map(|n| n.to_digit(10).unwrap() as i8).collect())
        .collect()
}

pub fn parse_navigation_subsystem(filename: &str) -> Vec<String> {
    let mut navigation_subsystem: Vec<String> = Vec::new();

    for line in build_reader(filename).lines() {
        navigation_subsystem.push(line.unwrap());
    }

    navigation_subsystem
}

pub fn parse_octopus_energy_levels(filename: &str) -> Vec<Vec<i8>> {
    let mut lines = Vec::new();

    for line in build_reader(filename).lines() {
        let line = line.unwrap().chars().map(|n| n.to_digit(10).expect("Not a number") as i8).collect::<Vec<i8>>();
        lines.push(line);
    }

    lines
}

pub fn parse_cave_paths(filename: &str) -> HashMap<String, Vec<String>> {
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();

    for line in build_reader(filename).lines() {
        let line = line.unwrap();
        let l: Vec<&str> = line.split("-").collect();

        paths.entry(String::from(l[0])).or_default().push(String::from(l[1]));
    }

    paths
}

pub fn parse_manual_instructions(filename: &str) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut points = Vec::new();
    let mut folds = Vec::new();
    let reader = build_reader(filename);
    let mut half_instructions = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            half_instructions = true;
            continue;
        }

        // Read coordinates
        if !half_instructions {
            let mut point = line.split(',');
            points.push((point.next().unwrap().parse::<usize>().unwrap(), point.next().unwrap().parse::<usize>().unwrap()));
        }
        // Read folds
        else {
            let mut line = line.replace("fold along ", "");
            let axis = line.chars().nth(0).unwrap();

            if axis == 'x' {
                line = line.replace("x=", "");
                folds.push((line.parse::<usize>().unwrap(), 0));
            }
            else if axis == 'y' {
                line = line.replace("y=", "");
                folds.push((0, line.parse::<usize>().unwrap()));
            }
            else {
                panic!("Hahaha... NO.");
            }
        }
    }

    (points, folds)
}

pub fn parse_polymer_instructions(filename: &str) -> (String, HashMap<String, char>) {
    let mut reader = build_reader(filename);

    // Template
    let mut first_line = String::new();
    reader.read_line(&mut first_line).expect("Could not read line");
    let template = String::from(first_line.trim());

    // Pair insertion
    let mut pairs: HashMap<String, char> = HashMap::new();

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let split = line.split(" -> ").collect::<Vec<&str>>();
        pairs.insert(String::from(split[0]), split[1].chars().collect::<Vec<char>>()[0]);
    }

    (template, pairs)
}

pub fn parse_cave_risk_level(filename: &str) -> Vec<Vec<i64>> {
    let mut lines = Vec::new();

    for line in build_reader(filename).lines() {
        let line = line.unwrap().chars().map(|n| n.to_digit(10).expect("Not a number") as i64).collect::<Vec<i64>>();
        lines.push(line);
    }

    lines
}

pub fn parse_hexadecimal_transmission(filename: &str) -> String {
    fs::read_to_string(filename).expect("Error reading file").trim().to_string()
}

pub fn parse_target_area(filename: &str) -> ((i32, i32), (i32, i32)) {
    let line = fs::read_to_string(filename)
        .expect("Error reading file")
        .trim()
        .to_string()
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .collect::<String>();
    let line = line.split(", y=").collect::<Vec<&str>>();
    // (x1, x2)
    let xs = line[0].split("..").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // (y1, y2)
    let ys= line[1].split("..").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    ((xs[0], xs[1]), (ys[0], ys[1]))
}

pub fn parse_snailfish_homework(filename: &str) -> Vec<String> {
    build_reader(filename).lines().map(|line| line.unwrap()).collect()
}

pub fn parse_algorithm_and_image(filename: &str) -> (Vec<char>, Vec<Vec<char>>) {
    // Optimization: true and false
    let mut reader = build_reader(filename);

    let mut first_line = String::new();
    reader.read_line(&mut first_line).expect("Could not read line");
    let algorithm = first_line.trim().chars().collect::<Vec<_>>();

    let mut image: Vec<Vec<char>> = Vec::new();

    for line in reader.lines().skip(1) {
        let line_str = line.unwrap();
        
        image.push(line_str.trim().chars().collect::<Vec<char>>());
    }

    (algorithm, image)
}

pub fn parse_starting_positions(filename: &str) -> Vec<u16> {
    // Length of the useless part from the input to skip it
    let len_str = String::from("Player 1 starting position: ").len();
    
    build_reader(filename).lines().map(|line| line.unwrap()[len_str..].to_string().parse::<u16>().unwrap()).collect::<Vec<u16>>()
}

pub fn parse_cuboids(filename: &str) -> Vec<(bool, Vec<Vec<i32>>)> {
    let mut status_points = Vec::new();

    for line in build_reader(filename).lines() {
        let l = line.unwrap();
        let l = l.split_whitespace().collect::<Vec<&str>>();
        let status = if l[0] == "on" { true } else { false };
        let points = l[1].split(",").map(|p| p[2..].split("..").map(|n| n.to_string().parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
        status_points.push((status, points));
    }

    status_points
}

pub fn parse_program(filename: &str) -> Vec<Vec<String>> {
    build_reader(filename).lines().map(|line| line.unwrap().split_whitespace().map(|s| String::from(s)).collect()).collect()
}

pub fn parse_sea_cucumbers(filename: &str) -> Vec<Vec<char>> {
    build_reader(filename).lines().map(|line| line.unwrap().chars().collect()).collect()
}
