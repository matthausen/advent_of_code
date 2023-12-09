/* 
    Day 07: Camel Cards: UNSOLVED
    
    Example:
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
*/
use std::{fs, collections::HashMap};

const FILEPATH: &str = "src/bin/day07_input.txt";

#[derive(Debug)]
struct CamelGameData{
    hand: String,
    bid: i32,
    score: i32, 
}

fn read_from_input() -> String {
    let file_content = fs::read_to_string(FILEPATH)
        .expect("Error reading the file");
    return file_content
}

fn order_hands_by_score(input: &str) -> Vec<CamelGameData> {
    let score_dict: HashMap<String, i32> = [
        ("2".to_string(), 2),
        ("3".to_string(), 3),
        ("4".to_string(), 4),
        ("5".to_string(), 5),
        ("6".to_string(), 6),
        ("7".to_string(), 7),
        ("8".to_string(), 8),
        ("9".to_string(), 9),
        ("T".to_string(), 10),
        ("J".to_string(), 11),
        ("Q".to_string(), 12),
        ("K".to_string(), 13),
        ("A".to_string(), 14),
    ]
    .iter()
    .cloned()
    .collect();
    
    let mut ordered_games: Vec<CamelGameData> = Vec::new();


    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let hand_score: i32 = parts[0]
                .chars()
                .map(|c| *score_dict.get(&c.to_string()).unwrap_or(&0))
                .sum();
            ordered_games.push(CamelGameData { hand: parts[0].to_string(), bid: parts[1].parse().unwrap(), score: hand_score });
        }
    }

    ordered_games.sort_by(|a, b| b.score.cmp(&a.score));

    ordered_games
}

fn main() {
    let input = read_from_input();

    let ordered_hands = order_hands_by_score(&input);

    let mut sum = 0;
    
    for (index, game) in ordered_hands.iter().enumerate() {
        let score = (game.bid * (index as i32 +1));
        sum +=score;
    }

    println!("Result is {:#?}", sum);
}