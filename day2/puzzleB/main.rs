use std::fs::File;
use std::io::Read;

fn main() {
    println!("ADVENT OF CODE 2023 DAY 2 - A");

    let mut file = File::open("../puzzle.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let mut sum = 0;

    for line in contents.lines() {

            let mut number = String::new();
            let mut nbr: u32 = 0;
            let mut nbr_red: u32 = 0;
            let mut nbr_green: u32 = 0;
            let mut nbr_blue: u32 = 0;

            let mut next = false;

            for c in line.chars() {

                if next {
                   if c.is_whitespace() {
                       next = false;
                   }
                } else if c.is_digit(10) {
                    number.push(c);
                } else if c.is_whitespace() {
                    if !number.is_empty() {
                        nbr = number.parse::<u32>().unwrap_or(0);
                        number.clear();
                    }
                } else if c == 'r' {
                    if nbr_red == 0 {
                        nbr_red = nbr;
                    } else if nbr > nbr_red{
                        nbr_red = nbr;
                    }
                    next = true;
                } else if c == 'g' {
                    if nbr_green == 0 {
                        nbr_green = nbr;
                    } else if nbr > nbr_green {
                        nbr_green = nbr;
                    }
                    next = true;
                } else if c == 'b' {
                    if nbr_blue == 0 {
                        nbr_blue = nbr;
                    } else if nbr > nbr_blue {
                        nbr_blue = nbr;
                    }
                    next = true;
                }

            }

            let power_line = nbr_red * nbr_green * nbr_blue;
            sum += power_line;
        }

        println!("Sum: {}", sum);
    }
