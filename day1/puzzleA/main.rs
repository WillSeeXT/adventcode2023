use std::fs::File;
use std::io::Read;

fn main() {
    println!("ADVENT OF CODE 2023 DAY 1 - A");

    let mut file = File::open("puzzle.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let mut sum: u32 = 0;

    for line in contents.lines() {
        let mut first_digit: char = ' ';
        let mut second_digit: char = ' ';

        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit == ' ' {
                    first_digit = c;
                } else {
                    second_digit = c;
                }
            }
        }

        if second_digit == ' ' {
          second_digit = first_digit;
        }

        let concatenated_digits = format!("{}{}", first_digit, second_digit);
        // println!("{} + {} = {}", first_digit, second_digit, concatenated_digits);
        sum += concatenated_digits.parse::<u32>().unwrap();
    }

    println!("Sum: {}", sum);
}
