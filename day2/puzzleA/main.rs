use std::fs::File;
use std::io::Read;

fn main() {
    println!("ADVENT OF CODE 2023 DAY 2 - A");

    let mut file = File::open("../puzzle.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let mut sum = 0;

    for line in contents.lines() {
        let game_index = line.find("Game ").unwrap_or(0) + "Game ".len();
        let id = line[game_index..].chars().take_while(|c| c.is_digit(10)).collect::<String>();

            let mut number = String::new();
            let mut nbr: u32 = 0;
            let mut done = false;
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
                    if nbr > 12 {
                        done = true;
                        println!("BAD {} Red: {}", id, nbr);
                    }
                    next = true;
                } else if c == 'g' {
                    if nbr > 13 {
                        done = true;
                        println!("BAD {} Green: {}", id, nbr);
                    }
                    next = true;
                } else if c == 'b' {
                    if nbr > 14 {
                        done = true;
                        println!("BAD {} Blue: {}", id, nbr);
                    }
                    next = true;
                }

                if done {
                    break;
                }

            }

            if !done {
                sum += id.parse::<u32>().unwrap_or(0);
            }
        }

        println!("Sum: {}", sum);
    }
