/* 
    Day 08: Haunted Wasteland: SOLVED?
    
    Example:

    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
*/

use std::fs;

const FILEPATH: &str = "src/bin/day08_input.txt";

#[derive(Debug)]
struct Direction {
    start: String,
    directions: String,
}


fn read_from_input() -> String {
    let file_content = fs::read_to_string(FILEPATH)
        .expect("Error reading the file");
    return file_content
}


fn read_directions(input: &str) -> String {
    input.lines().next().unwrap_or_default().trim().to_string()
}


fn list_locations(input: &str) -> Vec<Direction> {
    let mut locations: Vec<Direction> = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let key = parts[0];
            let value = parts[1].trim_matches(|c| c == '(' || c == ')' || c == ' ').to_string();
            locations.push(Direction{start: key.to_string(), directions: value})
        }
    }

    locations
}

fn calculate_directions(directions: String, locations: Vec<Direction>) -> i32 {
    let mut current_location = "RTF";
    let mut num_steps = 0;

    // println!("Current location {:#?}", current_location);

    while current_location != "ZZZ" {
        for d in directions.chars() {
            println!("Current location {:#?}", current_location);
            let next_destination = locations.iter().find(|l| l.start == current_location);
            match next_destination {
                Some(l) => {
                    let possible_directions:Vec<_> = l.directions.split(", ").collect();
           
                    if d == 'L' {
                        current_location = possible_directions[0];
                        num_steps+=1;
                    }
                    if d == 'R' {
                        current_location = possible_directions[1];
                        num_steps+=1;
                    }

                }
                None => println!("could not find next directions")
            }
        }
    }
    
    return num_steps;
}

fn main() {
    let input = read_from_input();

    let directions = read_directions(&input);
    let locations = list_locations(&input);

   
    let result = calculate_directions(directions, locations);
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT:&str = 
        "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_calculate_directions() {
        let directions = read_directions(&INPUT);
        let locations = list_locations(&INPUT);

        let res = calculate_directions(directions, locations);
        assert_eq!(res, 2);
    }

    const INPUT_2:&str = 
        "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
    
    #[test]
    fn test_calculate_directions_2() {
        let directions = read_directions(&INPUT_2);
        let locations = list_locations(&INPUT_2);

        let res = calculate_directions(directions, locations);
        assert_eq!(res, 6);
    }
    
}
