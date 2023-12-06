use std::fs::File;
use std::io::{self, BufRead, Seek};

fn main() {
    let puzzle_path = "../puzzle.txt";
    // let puzzle_path = "../puzzleTest.txt";
    let file = File::open(&puzzle_path).unwrap();

    let time = extract_time(&file);
    let distance = extract_distance(&file);

    let mut win_cnt = 0;

    for i in 0..time {
        if (time-i)*i > distance {
            win_cnt += 1;
        }
    }

    println!("Win count: {}", win_cnt);

}

fn extract_time(mut file: &File) -> u64 {
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("Time:") {
                let time: u64 = line
                    .split_whitespace()
                    .skip(1)
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);
                file.rewind().unwrap();
                return time;
            }
        }
    }

    0
}

fn extract_distance(mut file: &File) -> u64 {
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("Distance:") {
                let distance: u64 = line
                    .split_whitespace()
                    .skip(1)
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);
                file.rewind().unwrap();
                return distance;
            }
        }
    }

    0
}
