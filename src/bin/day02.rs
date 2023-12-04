/* 
    Cube Conundrum: SOLVED
*/
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

const FILEPATH: &str = "src/bin/day02_input.txt";

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

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

fn extract_game_id(s: &str) -> Option<u32> {
    let mut result = String::new();

    for c in s.chars() {
        if c.is_digit(10) {
            result.push(c);
        }
    }

    result.parse::<u32>().ok()
}

fn check_game_configuration(game: &str) -> bool {
    let mut is_possible = true;

    let games: Vec<&str> = game.split(";").collect();
    for game in games {
        
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;
        
        for part in game.split(',') {
            let mut iter = part.trim().split_whitespace();
            let count: u32 = iter.next().unwrap_or("0").parse().unwrap_or(0);
            let color = iter.next().unwrap_or("").to_lowercase();
            
            match color.as_str() {
                "red" => red_count += count,
                "green" => green_count += count,
                "blue" => blue_count += count,
                _ => {} // Ignore unknown colors
            }
        }
        if red_count > MAX_RED as u32 || green_count > MAX_GREEN as u32 || blue_count  > MAX_BLUE as u32 {
            is_possible = false
        }
    }
    is_possible
}


fn main() -> std::io::Result<()> {
    match read_from_input() {
        Ok(lines) => {
            let mut ids_sum = 0;
            for line in lines {
                let split_str: Vec<&str> = line.split(":").collect();
                let game_id = extract_game_id(split_str[0]);
                let game = split_str[1];

                match game_id {
                    Some(id) =>{
                        if check_game_configuration(game) {
                            ids_sum += id;
                        }
                    }
                    None => {
                        println!("No game id was found")
                    }
                }
                println!("SUM OF ID IS: {}", ids_sum);
            }

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
    fn test_extract_game_id() {
        assert_eq!(extract_game_id("Game 42"), Some(42));
        assert_eq!(extract_game_id("Game 101"), Some(101));
        assert_eq!(extract_game_id("No game id here"), None);
    }

    #[test]
    fn test_check_game_configuration() {
        assert_eq!(check_game_configuration("10 red, 8 green, 7 blue; 10 red, 8 green, 7 blue"), true);
        assert_eq!(check_game_configuration("5 red, 4 green, 12 blue; 10 red, 8 green, 7 blue"), true);

        assert_eq!(check_game_configuration("100 red, 8 green, 70 blue; 10 red, 8 green, 7 blue"), false);
        assert_eq!(check_game_configuration("5 red, 4 green, 12 blue; 90 red, 82 green, 17 blue"), false);
    }
}
