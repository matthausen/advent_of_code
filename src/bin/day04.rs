/* 
    Scratchcards: SOLVED
*/
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

const FILEPATH: &str = "src/bin/day04_input.txt";


fn read_from_input() -> Result<Vec<String>> {
    let mut lines = vec![];

    let file = File::open(FILEPATH)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}

fn string_to_card_numbers(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn find_matching_numbers(arr1: &[u32], arr2: &[u32]) -> Vec<u32> {
    let mut matching_numbers: Vec<u32> = Vec::new();

    for &i in arr1 {
        for &j in arr2 {
            if i == j {
                matching_numbers.push(i);
            }
        }
    }
    return matching_numbers
}

fn calculate_score(matching_numbers_length: usize ) -> usize {
    if matching_numbers_length > 1 {
        let y = (matching_numbers_length - 1) as u32;
        return usize::pow(1 * 2, y);
    }

    matching_numbers_length
}


fn main() -> std::io::Result<()> {
    match read_from_input() {
        Ok(lines) => {
            let mut total_score = 0;
            for line in lines {
                let split_str: Vec<&str> = line.split(":").collect();
                let scorecards = split_str[1];
                let numbers: Vec<&str> = scorecards.split("|").collect();

                // card winning numbers
                let card_winning_numbers = string_to_card_numbers(numbers[0]);

                // player numbers
                let players_numbers = string_to_card_numbers(numbers[1]);

                // matching numbers 
                let matching_numbers = find_matching_numbers(&card_winning_numbers, &players_numbers);
                total_score += calculate_score(matching_numbers.len());
            }
            println!("Total score {:#?}", total_score)
        }
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found");
                    return Ok(());
                }
                _ => return Err(error),
            }
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_card_numbers() {
        assert_eq!(string_to_card_numbers("41 48 83 86 17"), vec![41, 48, 83, 86, 17]);
        assert_eq!(string_to_card_numbers("13 32 20 16 61"), vec![13, 32, 20, 16, 61]);
        assert_eq!(string_to_card_numbers("1 21 53 59 44"), vec![1, 21, 53, 59, 44]);
        assert_eq!(string_to_card_numbers("41 92 73 84 69"), vec![41, 92, 73, 84, 69]);
        assert_eq!(string_to_card_numbers("87 83 26 28 32"), vec![87, 83, 26, 28, 32]);
        assert_eq!(string_to_card_numbers("31 18 13 56 72"), vec![31, 18, 13, 56, 72]);
    }

    #[test]
    fn test_find_matching_numbers() {
        assert_eq!(find_matching_numbers(&[41, 48, 83, 86, 17], &[83, 86, 6,31,17,9, 48, 53]), vec![48, 83, 86, 17]);
    }

    #[test]
    fn test_calculate_score() {
        assert_eq!(calculate_score(1), 1);
        assert_eq!(calculate_score(0), 0);
        assert_eq!(calculate_score(4), 8);
    }
}
