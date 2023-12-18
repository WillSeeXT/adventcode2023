
const FROM_LEFT: u64 = 0;
const FROM_RIGHT: u64 = 1;
const FROM_UP: u64 = 2;
const FROM_DOWN: u64 = 3;

fn main() {
    let puzzle = std::fs::read_to_string("../puzzle.txt").unwrap();
    let answer = solve(puzzle);
    println!("Answer{}", answer);
}

fn solve(puzzle: String) -> u32 {
    // let first_line = puzzle.lines().next().unwrap_or("").trim();
    // let num_chars = first_line.chars().filter(|&c| !c.is_whitespace()).count();

    let mut start_point: Option<(usize, usize)> = None; // Initialize start point as None
    let mut map: Vec<Vec<char>> = Vec::new();

    let max_x = puzzle.lines().count() as usize;
    let max_y = puzzle.lines().next().unwrap_or("").len() as usize;

    // println!("Max x: {}, Max y: {}", max_x, max_y);

    for (row_index, line) in puzzle.lines().enumerate() {
        let chars: Vec<char> = line.chars().filter(|&c| !c.is_whitespace()).collect();


        map.push(chars.clone());

        // Find the starting point
        if let Some(column_index) = chars.iter().position(|&c| c == 'S') {
            start_point = Some((row_index, column_index));
        }
    }

    // for row in &map {
    //     for &c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }


    if start_point.unwrap().1 > 0{
        let value = map[start_point.unwrap().0][start_point.unwrap().1 - 1];

        if value == 'L' || value == '-' || value == 'F' {
            println!("Left of start point is {}", value);
            down_the_rabbit_hole(&map, (start_point.unwrap().0,start_point.unwrap().1 - 1), FROM_RIGHT);
        }
    }

    if start_point.unwrap().1 < max_y {
        let value = map[start_point.unwrap().0][start_point.unwrap().1 + 1];

        if value == 'J' || value == '-' || value == '7' {
            println!("Right of start point is {}", value);
            down_the_rabbit_hole(&map, (start_point.unwrap().0,start_point.unwrap().1 + 1), FROM_LEFT);
        }

    }

    if start_point.unwrap().0 > 0 {
        let value = map[start_point.unwrap().0 - 1][start_point.unwrap().1];

        if value == 'F' || value == '|' || value == '7' {
            println!("Upper of start point is {}", value);
            down_the_rabbit_hole(&map, (start_point.unwrap().0 - 1,start_point.unwrap().1), FROM_DOWN);
        }
    }

    if start_point.unwrap().0 < max_x {
        let value = map[start_point.unwrap().0 + 1][start_point.unwrap().1];

        if value == 'L' || value == '|' || value == 'J' {
            println!("Bottom start point is {}", value);
            down_the_rabbit_hole(&map, (start_point.unwrap().0 + 1,start_point.unwrap().1), FROM_UP);
        }
    }



    // if let Some(coord) = map.get(2).and_then(|row| row.get(2)) {
    //     println!("Coordinate at (2, 2): {}", coord);
    // }

    0 // Placeholder return value
}

fn down_the_rabbit_hole(map: &Vec<Vec<char>>, start_point: (usize, usize), from: u64) -> u64 {
    let mut current_point = start_point;
    let mut from_direction = from;
    let mut steps_cnt = 0;

    // println!("LEFT: {}, RIGHT: {}, UP: {}, DOWN: {}", map[current_point.0][current_point.1 - 1], map[current_point.0][current_point.1 + 1], map[current_point.0 - 1][current_point.1], map[current_point.0 + 1][current_point.1]);

    // GO LEFT : map[current_point.0][current_point.1 - 1]
    // GO RIGHT : map[current_point.0][current_point.1 + 1]
    // GO UP : map[current_point.0 - 1][current_point.1]
    // GO DOWN : map[current_point.0 + 1][current_point.1]

    loop {
        //Get the current symbol
        let current_symbol = map[current_point.0][current_point.1];
        println!("Current symbol: {}", current_symbol);


        // println!("Diff x: {}, Diff y: {}", diff_x, diff_y);

        // break;

        match current_symbol {
            'L' => {
                if from_direction == FROM_RIGHT {
                    // Go up
                    current_point.0 -= 1;
                    from_direction = FROM_DOWN
                } else if from_direction == FROM_UP {
                    // Go right
                    current_point.1 += 1;
                    from_direction = FROM_LEFT
                } else {
                    println!("L Error: from_direction is not valid");
                    break;
                }
                steps_cnt += 1;
            },
            'J' => {
                if from_direction == FROM_LEFT {
                    // Go up
                    current_point.0 -= 1;
                    from_direction = FROM_DOWN
                } else if from_direction == FROM_UP {
                    // Go left
                    current_point.1 -= 1;
                    from_direction = FROM_RIGHT
                } else {
                    println!(" J Error: from_direction is not valid");
                    break;
                }
                steps_cnt += 1;
            },
            'F' => {
                if from_direction == FROM_RIGHT {
                    // Go down
                    current_point.0 += 1;
                    from_direction = FROM_UP
                } else if from_direction == FROM_DOWN {
                    // Go right
                    current_point.1 += 1;
                    from_direction = FROM_LEFT
                } else {
                    println!(" F Error: from_direction is not valid");
                    break;
                }
                steps_cnt += 1;
            },
            '7' => {
                if from_direction == FROM_LEFT {
                    // Go down
                    current_point.0 += 1;
                    from_direction = FROM_UP
                } else if from_direction == FROM_DOWN {
                    // Go left
                    current_point.1 -= 1;
                    from_direction = FROM_RIGHT
                } else {
                    println!(" J Error: from_direction is not valid");
                    break;
                }
                steps_cnt += 1;
            },
            '|' => {
                if from_direction == FROM_DOWN {
                    // Go up
                    current_point.0 -= 1;
                    from_direction = FROM_DOWN
                } else if from_direction == FROM_UP {
                    // Go down
                    current_point.0 += 1;
                    from_direction = FROM_UP
                } else {
                    println!(" | Error: from_direction is not valid");
                    break;
                }
                steps_cnt += 1;
            },
            '-' => {
                if from_direction == FROM_LEFT {
                    // Go right
                    current_point.1 += 1;
                    from_direction = FROM_LEFT
                } else if from_direction == FROM_RIGHT {
                    // Go left
                    current_point.1 -= 1;
                    from_direction = FROM_RIGHT
                } else {
                    println!(" - Error: from_direction is not valid");
                    break;
                }
                steps_cnt += 1;
            },
            'S' => {
                let farthest_point = (steps_cnt as f32 / 2.0).ceil() as i32;
                println!("Found the start point again in {} steps FURTHER IS : {}", steps_cnt, farthest_point);
                break;
            },
            _ => {
                // We're at the end of the line
                break;
            }
        }

    }

    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let puzzle = String::from(
        "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF");
        assert_eq!(solve(puzzle), 4);
    }
}