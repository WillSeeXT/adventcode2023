use std::fs::File;
use std::io::{self, BufRead, Seek};

struct WinChart {
    time: u64,
    distance: u64,
}

fn main() {
    let puzzle_path = "../puzzle.txt";
    // let puzzle_path = "../puzzleTest.txt";
    let file = File::open(&puzzle_path).unwrap();

    let time = get_time_chart(&file);
    let distance = get_distance_chart(&file);

    let mut win_chart: Vec<WinChart> = Vec::new();
    let mut margin_win_cnt: Vec<u64> = Vec::new();


    for (index, time) in time.iter().enumerate() {
        win_chart.push(WinChart {
            time: *time,
            distance: distance[index],
        });

        let mut win_cnt = 0;

        for i in 0..*time {
            if (time-i)*i > distance[index] {
                win_cnt += 1;
            }
        }
        margin_win_cnt.push(win_cnt);
    }

    let mut margin_error = 1;

    for win in margin_win_cnt {
        margin_error *= win;
    }

    println!("Margin error: {}", margin_error);
    for win_chart in win_chart {
        println!("Time: {}, Distance: {}", win_chart.time, win_chart.distance);
    }

}

fn get_time_chart(mut file: &File) -> Vec<u64> {
    let mut time: Vec<u64> = Vec::new();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
                if line.starts_with("Time:") {
                    let numbers: Vec<u64> = line
                        .split_whitespace()
                        .skip(1)
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    time.extend(numbers);
                    break;
                }
            }
    }

    file.rewind().unwrap();
    time
}

fn get_distance_chart(mut file: &File) -> Vec<u64> {
    let mut distance: Vec<u64> = Vec::new();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
                if line.starts_with("Distance:") {
                    let numbers: Vec<u64> = line
                        .split_whitespace()
                        .skip(1)
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    distance.extend(numbers);
                    break;
                }
            }
    }

    file.rewind().unwrap();
    distance
}