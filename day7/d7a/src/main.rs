
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


struct Hand {
    cards: Vec<u32>,
    bet: u64,
    weight: u32,
}

fn main() {
    // let puzzle_path = "../puzzleTest.txt";
    let puzzle_path = "../puzzle.txt";
    let file = File::open(&puzzle_path).unwrap();

    let reader = io::BufReader::new(file);

    let mut hands: Vec<Hand> = Vec::new();

    let mut five_of_a_kind: Vec<Hand> = Vec::new();
    let mut four_of_a_kind: Vec<Hand> = Vec::new();
    let mut full_house: Vec<Hand> = Vec::new();
    let mut tree_of_a_kind: Vec<Hand> = Vec::new();
    let mut two_pairs: Vec<Hand> = Vec::new();
    let mut one_pair: Vec<Hand> = Vec::new();
    let mut high_card: Vec<Hand> = Vec::new();

    // Get all hands from file
    for line in reader.lines() {
        if let Ok(hand_str) = line {
            let current_hand = hand_str
                .split_whitespace()
                .next()
                .unwrap();

            //parse current hand to add each char a vector member
            let current_hand_vec: Vec<char> = current_hand
                .chars()
                .collect();

            let mut hand: Hand = Hand {
                cards: Vec::new(),
                bet: 0,
                weight: 0,
            };

            for card in current_hand_vec {
                match card {
                    '2' => hand.cards.push(2),
                    '3' => hand.cards.push(3),
                    '4' => hand.cards.push(4),
                    '5' => hand.cards.push(5),
                    '6' => hand.cards.push(6),
                    '7' => hand.cards.push(7),
                    '8' => hand.cards.push(8),
                    '9' => hand.cards.push(9),
                    'T' => hand.cards.push(10),
                    'J' => hand.cards.push(11),
                    'Q' => hand.cards.push(12),
                    'K' => hand.cards.push(13),
                    'A' => hand.cards.push(14),
                    _ => println!("Error: {} is not a valid card", card),
                }
            }

            hand.bet = hand_str
                .split_whitespace()
                .skip(1)
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap();

            hand.weight = get_hand_weight(&hand);
            hands.push(hand);
        }
    }

    // Now sort all hands to correct place
    for hand in hands {
        let mut sorted_hand = hand.cards.clone();
        sorted_hand.sort();

        // Count the occurrences of each number in sorted_hand
        let mut number_count: HashMap<u32, usize> = HashMap::new();
        for &num in &sorted_hand {
            *number_count.entry(num).or_insert(0) += 1;
        }

        // Check for five of a kind
        if number_count.values().any(|&x| x == 5) {
            five_of_a_kind.push(hand);
        }
        // Check for four of a kind
        else if number_count.values().any(|&x| x == 4) {
            four_of_a_kind.push(hand);
        }
        // Check for full house
        else if number_count.values().any(|&x| x == 3) && number_count.values().any(|&x| x == 2) {
            full_house.push(hand);
        }
        // Check for tree of a kind
        else if number_count.values().any(|&x| x == 3) {
            tree_of_a_kind.push(hand);
        }
        // Check for two pairs
        else if number_count.values().filter(|&x| *x == 2).count() == 2 {
            two_pairs.push(hand);
        }
        // Check for one pair
        else if number_count.values().any(|&x| x == 2) {
            one_pair.push(hand);
        }
        // Check for high card
        else {
            high_card.push(hand);
        }
    }

    let mut hand_cnt: u64 = 1;
    let mut total_win: u64 = 0;

    high_card.sort_by_key(|hand| hand.weight);
    let sorted_high_card: Vec<_> = high_card.into_iter().collect();
    for hand in sorted_high_card {
        println!("High card Hand: {:?}, Bet: {}, Rank: {}, Weight: {}", hand.cards, hand.bet, hand_cnt, hand.weight);
        total_win += hand.bet*hand_cnt;
        hand_cnt += 1;
    }

    one_pair.sort_by_key(|hand| hand.weight);
    let sorted_one_pairs: Vec<_> = one_pair.into_iter().collect();
    for hand in sorted_one_pairs {
        println!("One pair Hand: {:?}, Bet: {}, Rank: {}, Weight: {}", hand.cards, hand.bet, hand_cnt, hand.weight);
        total_win += hand.bet*hand_cnt;
        hand_cnt += 1;
    }
    println!("-------------------");
    println!("One pair Total win: {}", total_win);
    println!("-------------------");

    two_pairs.sort_by_key(|hand| hand.weight);
    let sorted_two_pairs: Vec<_> = two_pairs.into_iter().collect();
    for hand in sorted_two_pairs {
        println!("two pair Hand: {:?}, Bet: {}, Rank: {}, Weight: {}", hand.cards, hand.bet, hand_cnt, hand.weight);
        total_win += hand.bet*hand_cnt;
        hand_cnt += 1;
    }
    println!("-------------------");
    println!("Two pair Total win: {}", total_win);
    println!("-------------------");

    tree_of_a_kind.sort_by_key(|hand| hand.weight);
    let sorted_tree_of_a_kind: Vec<_> = tree_of_a_kind.into_iter().collect();
    for hand in sorted_tree_of_a_kind {
        println!("Tree of a kind Hand: {:?}, Bet: {}, Rank: {}, Weight: {}", hand.cards, hand.bet, hand_cnt, hand.weight);
        total_win += hand.bet*hand_cnt;
        hand_cnt += 1;
    }
    println!("-------------------");
    println!("Tree of a kind Total win: {}", total_win);
    println!("-------------------");

    full_house.sort_by_key(|hand| hand.weight);
    let sorted_full_house: Vec<_> = full_house.into_iter().collect();
    for hand in sorted_full_house {
        println!("Full house Hand: {:?}, Bet: {}, Rank: {}, Weight: {}", hand.cards, hand.bet, hand_cnt, hand.weight);
        total_win += hand.bet*hand_cnt;
        hand_cnt += 1;
    }
    println!("-------------------");
    println!("Full house Total win: {}", total_win);
    println!("-------------------");

    four_of_a_kind.sort_by_key(|hand| hand.weight);
    let sorted_four_of_a_kind: Vec<_> = four_of_a_kind.into_iter().collect();
    for hand in sorted_four_of_a_kind {
        println!("Four of a kind Hand: {:?}, Bet: {}, Rank: {}, Weight: {}", hand.cards, hand.bet, hand_cnt, hand.weight);
        total_win += hand.bet*hand_cnt;
        hand_cnt += 1;
    }
    println!("-------------------");
    println!("Four of a kind Total win: {}", total_win);
    println!("-------------------");

    five_of_a_kind.sort_by_key(|hand| hand.weight);
    let sorted_five_of_a_kind: Vec<_> = five_of_a_kind.into_iter().collect();
    for hand in sorted_five_of_a_kind {
        println!("Five of a kind Hand: {:?}, Bet: {}, Rank: {}, Weight: {}", hand.cards, hand.bet, hand_cnt, hand.weight);
        total_win += hand.bet*hand_cnt;
        hand_cnt += 1;
    }
    println!("-------------------");
    println!("Five of a kind Total win: {}", total_win);
    println!("-------------------");

    println!("TOTAL WIN: {}", total_win);
}


fn get_hand_weight(hand: &Hand)-> u32{
    let mut weight: u32 = 0;
    for (i, card) in hand.cards.iter().enumerate() {
        if i == 0 {
            weight += card * 100000000; //10⁸
        }
        else if i == 1 {
            weight += card * 1000000; //10⁶
        }
        else if i == 2 {
            weight += card * 10000; //10⁴
        }
        else if i == 3 {
            weight += card * 100; //10²
        }
        else if i == 4 {
            weight += card;
        }
    }

    weight
}

