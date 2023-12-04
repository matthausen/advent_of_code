/* 
    Day 03: Gear Ratios: UNSOLVED
*/
use std::fs;

const FILEPATH: &str = "src/bin/day03_input.txt";

fn read_from_input() -> String {
    let file_content = fs::read_to_string(FILEPATH)
        .expect("Error reading the file");
    return file_content
}

fn is_valid_position(row: isize, col: isize, height: isize, width: isize) -> bool {
    row >= 0 && col >= 0 && row < height && col < width
}

fn find_adjacent_numbers(grid: &Vec<Vec<char>>, row: isize, col: isize) -> Vec<char> {
    let mut adjacent_numbers = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            let new_row = row + i;
            let new_col = col + j;

            if is_valid_position(new_row, new_col, grid.len() as isize, grid[0].len() as isize)
                && (i, j) != (0, 0)
                && grid[new_row as usize][new_col as usize] != '.'
            {
                adjacent_numbers.push(grid[new_row as usize][new_col as usize]);
            }
        }
    }

    adjacent_numbers
}


fn main() {
    let input = read_from_input();

    let rows: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

        let mut sum = 0;
        let mut current_number = String::new();
    
        for i in 0..rows.len() {
            for j in 0..rows[0].len() {
                if rows[i][j] != '.' && rows[i][j] != '*' {
                    let adjacent_numbers = find_adjacent_numbers(&rows, i as isize, j as isize);
                    println!("{:#?}", adjacent_numbers);
    
                    for &num in &adjacent_numbers {
                        if num.is_digit(10) {
                            current_number.push(num);
                        } else if !current_number.is_empty() {
                            sum += current_number.parse::<u32>().unwrap();
                            current_number.clear();
                        }
                    }
                }
            }
        }
    
        if !current_number.is_empty() {
            sum += current_number.parse::<u32>().unwrap();
        }
    
        println!("Sum of adjacent numbers: {}", sum);

}