use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn main() {
    // let puzzle_path = "../puzzleTest3.txt";
    let puzzle_path = "../puzzle.txt";
    let file = File::open(&puzzle_path).unwrap();
    let reader = io::BufReader::new(file);

    // Regex pattern to match the node name, left, and right values
    let pattern = r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)";

    let mut nodes_names: Vec<String> = Vec::new();
    let mut nodes_right_str: Vec<String> = Vec::new();
    let mut nodes_left_str: Vec<String> = Vec::new();
    let mut nodes_right_idx: Vec<u32> = Vec::new();
    let mut nodes_left_idx: Vec<u32> = Vec::new();
    let mut end_idx: Vec<u32> = Vec::new();
    let mut start_idx: Vec<u32> = Vec::new();
    let mut line_cnt: u32 = 0;
    let mut is_first_line: bool = true;
    let mut directions: String = String::new();



    // Get all nodes names, left, and right values from file
    for line in reader.lines() {
        if let Ok(line) = line {
            if is_first_line {
                directions = line;
                is_first_line = false;
                continue;
            }
            let re = Regex::new(pattern).unwrap();
            if let Some(captures) = re.captures(&line) {
                if let Some(node_name) = captures.get(1) {
                    if node_name.as_str().ends_with("Z") {
                        end_idx.push(line_cnt);
                    }

                    if node_name.as_str().ends_with("A") {
                        start_idx.push(line_cnt);
                    }

                    line_cnt += 1;

                    nodes_names.push(node_name.as_str().to_string());
                }
                if let Some(left) = captures.get(2) {
                    nodes_left_str.push(left.as_str().to_string());
                }
                if let Some(right) = captures.get(3) {
                    nodes_right_str.push(right.as_str().to_string());
                }
            }
        }
    }

    for right in nodes_right_str {
        for (j, node_name) in nodes_names.iter().enumerate() {
            if right == *node_name {
                nodes_right_idx.push(j as u32);
            }
        }
    }

    for left in nodes_left_str {
        for (j, node_name) in nodes_names.iter().enumerate() {
            if left == *node_name {
                nodes_left_idx.push(j as u32);
            }
        }
    }

    // println!("Nodes Names: {:?}", nodes_names);
    // println!("Left Indices: {:?}", nodes_left_idx);
    // println!("Right Indices: {:?}", nodes_right_idx);

    // println!("Start Indices: {:?}", start_idx);
    // println!("End Indices: {:?}", end_idx);

    let mut nodes_steps_cnt: Vec<u64> = Vec::new();
    // Now go to each direction and search for the ZZZ, end_idx
    for (i, start) in start_idx.iter().enumerate() {
        let mut current_pos = *start;
        let mut step_cnt: u64 = 0;
        let mut find_end = false;

        println!("Start: {}", start);

        while !find_end {
            for ch in directions.chars() {
                step_cnt += 1;
                match ch {
                    'L' => {
                        current_pos = nodes_left_idx[current_pos as usize];
                    },
                    'R' => {
                        current_pos = nodes_right_idx[current_pos as usize];
                    },
                    _=> println!("Error: {} is not a valid direction", ch),
                }

                if end_idx.contains(&current_pos) {
                    nodes_steps_cnt.push(step_cnt);
                    find_end = true;
                    break;
                }
            }
        }
    }

    // Calculate the LCM of all elements in nodes_steps_cnt
    let lcm_result = nodes_steps_cnt.iter().fold(1, |acc, &x| num::integer::lcm(acc, x));

    println!("Lowest Common Multiple: {}", lcm_result);

}
