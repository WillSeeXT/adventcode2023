use std::fs::File;
use std::io::Read;

struct Digit {
    digit: char,
    idx: u32,
}

fn main() {
    println!("ADVENT OF CODE 2023 DAY 1 - B");

    let mut file = File::open("puzzle.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let mut sum: u32 = 0;

    let digit_char = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];



    for line in contents.lines() {
        let mut first_digit = Digit {
            digit: ' ',
            idx: 0,
        };

        let mut second_digit = Digit {
            digit: ' ',
            idx: 0,
        };

        for (_, digit) in digit_char.iter().enumerate() {
            line.match_indices(digit.0).for_each(|(idx, _)| {
                if first_digit.digit == ' ' {
                    first_digit.digit = digit.1;
                    first_digit.idx = idx as u32;
                } else {
                    if idx as u32 <= first_digit.idx {
                        first_digit.idx = idx as u32;
                        first_digit.digit = digit.1;
                    }
                }

                if second_digit.digit == ' ' {
                    second_digit.digit = digit.1;
                    second_digit.idx = idx as u32;
                } else {
                    if idx as u32 >= second_digit.idx {
                        second_digit.idx = idx as u32;
                        second_digit.digit = digit.1;
                    }
                }
            });
        }

        let mut idx = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit.digit == ' ' {
                    first_digit.digit = c;
                    first_digit.idx = idx;
                } else {
                    if idx <= first_digit.idx {
                        first_digit.digit = c;
                        first_digit.idx = idx;
                    }
                }

                if second_digit.digit == ' ' {
                    second_digit.digit = c;
                    second_digit.idx = idx;
                } else {
                    if idx >= second_digit.idx {
                        second_digit.digit = c;
                        second_digit.idx = idx;
                    }
                }
            }
            idx += 1;
        }

        let concatenated_digits = format!("{}{}", first_digit.digit, second_digit.digit);
        println!("{}", concatenated_digits.parse::<u32>().unwrap());
        sum += concatenated_digits.parse::<u32>().unwrap();
    }

    println!("Sum: {}", sum);
}
