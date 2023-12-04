use std::fs::File;
use std::io::Read;
use regex::Regex;
// use regex::Regex;

struct NumberPosition {
    nbr : u32,
    start: usize,
    end: usize,
}


fn find_numbers(line: &str) -> Vec<NumberPosition> {
    let mut numbers = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for capture in re.captures_iter(line) {
        let number = capture[0].parse().unwrap();
        let start_idx = capture.get(0).unwrap().start();
        let end_idx = capture.get(0).unwrap().end();
        numbers.push(NumberPosition {
            nbr: number,
            start: start_idx,
            end: end_idx-1,
        });
    }

    numbers
}

fn numbers_is_adjacent(number: &NumberPosition, position: usize) -> bool {
    // println!("number {} start {} end {} position {}", number.nbr, number.start, number.end, position);
    if number.start <= position && number.end >= position {
        println!("number on position {} number {} start {} end {}", position, number.nbr, number.start, number.end);
        true
    } else if number.start == position + 1 {
        println!("number on right position {} number {} start {} end {}", position, number.nbr, number.start, number.end);
        true
    } else if number.end == position - 1 {
        println!("number on left position {} number {} start {} end {}", position, number.nbr, number.start, number.end);
        true
    } else {
        false
    }
}

fn find_star_positions(line: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    for (index, character) in line.chars().enumerate() {
        if character == '*' {
            positions.push(index);
        }
    }
    positions
}

fn main() {
    let puzzle_path = "../puzzle.txt";
    // let puzzle_path = "../puzzleTest.txt";
    let mut file = File::open(puzzle_path).expect("Failed to open file");
    let mut puzzle = String::new();
    file.read_to_string(&mut puzzle).expect("Failed to read file");

    let mut sum = 0;

    for (line_number, line) in puzzle.lines().enumerate() {

        let star_positions = find_star_positions(line);
        // for position in &star_positions {
        //     // println!("Found '*' at line {}, position {}", line_number, position);
        // }

        for position in star_positions {
            // println!("Found '*' at line {}, position {}", line_number, position);
            let mut first_number = 0;
            let mut second_number = 0;

            // Check at line
            let numbers = find_numbers(puzzle.lines().nth(line_number).unwrap());
            for number in numbers {
                if numbers_is_adjacent(&number, position) {
                    // println!("Found number {} at line {}, position {}-{}", number.nbr, line_number, number.start, number.end);
                    if first_number == 0 {
                        first_number = number.nbr;
                    } else {
                        second_number = number.nbr;
                    }
                }
            }

            // Check upper line
            if line_number > 0 {
                let numbers = find_numbers(puzzle.lines().nth(line_number - 1).unwrap());
                for number in numbers {
                    if numbers_is_adjacent(&number, position) {
                        // println!("Found number {} at line {}, position {}-{}", number.nbr, line_number, number.start, number.end);
                        if first_number == 0 {
                            first_number = number.nbr;
                        } else {
                            second_number = number.nbr;
                        }
                    }
                }
            }

            // Check lower line
            if line_number < puzzle.lines().count() - 1 {
                let numbers = find_numbers(puzzle.lines().nth(line_number + 1).unwrap());
                for number in numbers {
                    if numbers_is_adjacent(&number, position) {
                        // println!("Found number {} at line {}, position {}-{}", number.nbr, line_number, number.start, number.end);
                        if first_number == 0 {
                            first_number = number.nbr;
                        } else {
                            second_number = number.nbr;
                        }
                    }
                }
            }



            if first_number != 0 && second_number != 0 {
                println!("Found two numbers {} {}", first_number, second_number);
                sum += first_number * second_number;
            }

        }
    }

    println!("Sum: {}", sum);
}