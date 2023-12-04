use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;
use regex::Regex;
use std::str::FromStr;

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

fn main() {
    // let puzzle_path = "../puzzleTest.txt";
    let puzzle_path = "../puzzle.txt";
    let file = File::open(&puzzle_path).unwrap();
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"Card\s*(\d+): (.*) \| (.*)").unwrap();
    let mut cards = Vec::new();
    let mut cards_nbr = Vec::new();
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(caps) = re.captures(&line) {
            let id = u32::from_str(&caps[1]).unwrap();
            let winning_numbers = caps[2].split_whitespace().map(|n| u32::from_str(n).unwrap()).collect();
            let card_numbers = caps[3].split_whitespace().map(|n| u32::from_str(n).unwrap()).collect();
            let card = Card { id, winning_numbers, card_numbers,};
            cards.push(card);

            let card_nbr: u32 = 1;
            cards_nbr.push(card_nbr);
        }
    }

    for i in 0..cards.len() {
        let card = &mut cards[i];
        println!("Card ID: {}", card.id);
        // println!("Winning numbers: {:?}", card.winning_numbers);
        // println!("Card numbers: {:?}", card.card_numbers);

        let mut points = 0;
        let mut matching_numbers = Vec::new();
        for number in &card.winning_numbers {
            if card.card_numbers.contains(number) {
                matching_numbers.push(*number);
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        if !matching_numbers.is_empty() {
            for (idx, _) in matching_numbers.iter().enumerate() {
                let card_idx = card.id+(idx as u32 +1);
                cards_nbr[card_idx as usize] += 1*cards_nbr[card.id as usize];
                println!("  Idx: {} cards_nbr: {}",card_idx, cards_nbr[card_idx as usize]);
            }
        }
    }

    for (idx, nbr) in cards_nbr.iter().enumerate() {
        // println!("Idx: {} cards_nbr: {}", idx, nbr);
        sum += nbr;
    }
    println!("Sum: {}", sum);
}

