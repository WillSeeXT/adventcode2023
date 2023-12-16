use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let puzzle_path = "../puzzle.txt";
    // let puzzle_path = "../puzzleTest.txt";
    let file = File::open(&puzzle_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut final_sum: i32 = 0;

    for line in reader.lines() {
        let numbers: Vec<i32> = line.unwrap()
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();

        let mut sequence = numbers.clone();
        let mut og_sequence = numbers.clone();

        let mut diff_sequences: Vec<i32> = Vec::new();

        loop {
            match find_common_difference(&sequence) {
                Some(diff) => {
                    // println!("Common Difference: {}", diff);

                    // diff_sequences.push(diff);
                    let mut new_val = diff;

                    // println!("diff_sequences: {:?}", diff_sequences);
                    for val in diff_sequences.iter().rev() {

                        new_val = val - new_val;
                        println!("val: {} new_val {}", val, new_val);
                    }

                    println!("New val : {} first {}",new_val,og_sequence[0]);
                    // println!("Extrapolated value: {}", new_val + *og_sequence.last_mut().unwrap());
                    final_sum += og_sequence[0] - new_val;
                    break;
                }
                None => {
                    let new_sequence = generate_new_sequence(&sequence);
                    if new_sequence.is_empty() {
                        println!("Not an arithmetic sequence");
                        break;
                    }
                    sequence = new_sequence;
                    diff_sequences.push(sequence[0]);
                }
            }
        }

        println!("Final Sum: {}", final_sum);
        println!("------------------");
    }
}

fn find_common_difference(sequence: &[i32]) -> Option<i32> {
    // Ensure there are at least two terms in the sequence
    if sequence.len() < 2 {
        return None;
    }

    // Calculate the difference between the first two terms
    let initial_difference = sequence[1] - sequence[0];

    // Check the consistency of the difference throughout the sequence
    for i in 1..sequence.len() - 1 {
        if sequence[i + 1] - sequence[i] != initial_difference {
            // If differences are inconsistent, return None
            return None;
        }
    }

    // If differences are consistent, return the common difference
    Some(initial_difference)
}

fn generate_new_sequence(sequence: &[i32]) -> Vec<i32> {
    let mut new_sequence = Vec::new();

    for i in 0..sequence.len() - 1 {
        new_sequence.push(sequence[i + 1] - sequence[i]);
    }

    new_sequence
}

