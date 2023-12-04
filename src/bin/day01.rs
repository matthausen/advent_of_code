/* 
    Trebuchet: SOLVED
*/

use std::fs::File;
use std::io::{BufReader, BufRead, Result};

const FILEPATH: &str = "src/bin/day01_input.txt";

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

fn extrapolate_calibration_values_from_line(lines: Vec<&str>) -> Vec<Option<u32>> {
    lines
        .iter()
        .map(|&word| {
            let mut first_digit: Option<char> = None;
            let mut last_digit: Option<char> = None;

            for c in word.chars() {
                if c.is_digit(10) {
                    if first_digit.is_none() {
                        first_digit = Some(c);
                    }
                    last_digit = Some(c);
                }
            }

            if first_digit.is_none() {
                None
            } else {
                let result_str: String = vec![first_digit.unwrap(), last_digit.unwrap()].into_iter().collect();
                let result = result_str.parse::<u32>().unwrap();
                Some(result)
            }
        })
        .collect()
}


fn main() -> std::io::Result<()> {
    match read_from_input() {
        Ok(lines) => {
            let mut total = 0;

            let words = lines.iter().map(|s| s.as_str()).collect();
            
            let calibration_values = extrapolate_calibration_values_from_line(words);
            for value in calibration_values {
                match value {
                    Some(v) => {
                        total += v;
                    },
                    None => println!("could not extract calibration value from line")
                }
                println!{"Sum of calibration values is {}", total}
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
    fn test_extrapolate_calibration_values_from_line() {
        let lines = vec!["ahjdhd6nas8nans7", "dwidehfe6ksk", "nmnm4nmndec7"];
        let result = extrapolate_calibration_values_from_line(lines);
        assert_eq!(result, vec![Some(67), Some(66), Some(47)]);
    }
}
