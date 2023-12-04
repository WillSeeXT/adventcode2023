use std::fs::File;
use std::io::Read;
use regex::Regex;

fn is_special_char(ch: char) -> bool {
    !ch.is_digit(10) && ch != '.'
}

fn main() {
    let puzzle_path = "../puzzle.txt";
    // let puzzle_path = "../puzzleTest.txt";
    let mut file = File::open(puzzle_path).expect("Failed to open file");
    let mut puzzle = String::new();
    file.read_to_string(&mut puzzle).expect("Failed to read file");

    let number_regex = Regex::new(r"\d+").unwrap();

    let mut sum = 0;

    for (line_number, line) in puzzle.lines().enumerate() {
        let numbers: Vec<(i32, usize)> = number_regex.find_iter(line)
            .map(|m| (m.as_str().parse().unwrap(), m.start()))
            .collect();

        for (number, start) in numbers {
            //Start by checking each sides then check upper line if not first  and then check lower line if not last
            let line_chars: Vec<char> = line.chars().collect();
            let number_len = number.to_string().len();
            let line_max = line.len()-1;
            let  mut is_valid = false;
            let end = start+number_len-1;

            // println!("number {} start {} end {} line_max {}", number, start, end, line_max);
            //Check Left
            if start > 0 {
                if is_special_char(line_chars[start - 1]) {
                    is_valid = true;
                    // println!("Left: {}", line_chars[start - 1]);
                }
            }

            //Check Right
            if end < line_max {
                if is_special_char(line_chars[end+1]) {
                    is_valid = true;
                    // println!("Right: {}", line_chars[end+1]);
                }
            }

            //Check Upper
            if line_number != 0 {
                let upper_line = puzzle.lines().nth(line_number - 1).unwrap();
                let upper_line_chars: Vec<char> = upper_line.chars().collect();
                let mut upper_start = start;
                let mut upper_end = end;

                if start > 0 {
                    upper_start -= 1;
                }

                if upper_end < line_max {
                    upper_end += 1;
                }

                for i in upper_start..upper_end+1 {
                    // println!("Upper: {}", upper_line_chars[i]);
                    if is_special_char(upper_line_chars[i]) {
                        is_valid = true;
                    }
                }
            }

            //Check Lower
            if line_number != puzzle.lines().count()-1 {
                let lower_line = puzzle.lines().nth(line_number + 1).unwrap();
                let lower_line_chars: Vec<char> = lower_line.chars().collect();
                let mut lower_start = start;
                let mut lower_end = end;

                if start > 0 {
                    lower_start -= 1;
                }

                if lower_end < line_max {
                    lower_end += 1;
                }
                // println!("Lower start {} end {}", lower_start, lower_end);

                for i in lower_start..lower_end+1 {
                    // println!("Lower: {}", lower_line_chars[i]);
                    if is_special_char(lower_line_chars[i]) {
                        is_valid = true;
                    }
                }
            }

            if is_valid {
                sum += number;
                // println!("Valid: {}", number);
            }

        }
    }

    println!("Sum: {}", sum);
}
