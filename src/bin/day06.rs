/* 
    Day 06: Wait For It: SOLVED

    Distance = Speed * Time 

    time t 
    record distance r 

    button time = range 0..t (speed)
    motiontime mt = t - button time 
    distance d = button time * (motiontime)

    distance > r 
*/

use std::fs;

const FILEPATH: &str = "src/bin/day06_input.txt";

#[derive(Debug)]
struct BoatRaceData {
    time: i32,
    record: i32
}

fn read_from_input() -> String {
    let file_content = fs::read_to_string(FILEPATH)
        .expect("Error reading the file");
    return file_content
}

fn generate_boat_races(times:Vec<i32>, distances: Vec<i32>) -> Vec<BoatRaceData> {
    let mut boat_races: Vec<BoatRaceData> = Vec::new();

    for (i,t) in times.iter().enumerate() {
        for (j, r) in distances.iter().enumerate() {
            if i == j {
                let boat_race:BoatRaceData = BoatRaceData { time: *t, record: *r };
                boat_races.push(boat_race);
            }
        }
    }
    boat_races
}

fn calculate_possible_record(boat_races: Vec<BoatRaceData>) -> Vec<i32>{

    let mut winning_configuration = Vec::new();

    for boat_race in boat_races {
        
        let mut new_record = 0;

        for button_time in 0..boat_race.time+1 {
            let motion_time = boat_race.time - button_time;
            let distance  = button_time * motion_time;

            if distance > boat_race.record {
                new_record += 1;
            }
        }
        winning_configuration.push(new_record);
    }
    winning_configuration
}

fn main() {
    let input = read_from_input();
    let mut times = Vec::new();
    let mut distances = Vec::new();
    
    for (index, line) in input.lines().enumerate() {
        let values: Vec<i32> =  line.split_whitespace()
            .filter_map(|n| n.parse()
            .ok())
            .collect();

            if index == 0 {
                times.extend_from_slice(&values);
            }

            if index == 1 {
                distances.extend_from_slice(&values);
            }
            
    }

    let boat_races = generate_boat_races(times, distances);
    let possible_records = calculate_possible_record(boat_races);

    println!("Result - {:?}", possible_records);

}


#[cfg(test)]
mod tests {
    use super::*;

    const input:&str = "Time:      7  15   30
                        Distance:  9  40  200";

    #[test]
    fn test_generate_boat_races() {
        let times:Vec<i32> = vec![7, 15, 30];
        let records:Vec<i32> = vec![9, 40, 200];

        let res = generate_boat_races(times, records);
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].time, 7);
        assert_eq!(res[0].record, 9);
        assert_eq!(res[1].time, 15);
        assert_eq!(res[1].record, 40);
        assert_eq!(res[2].time, 30);
        assert_eq!(res[2].record, 200);
    }


    #[test]
    fn test_calculate_possible_record() {
        let boat_race_1: BoatRaceData = BoatRaceData { time: 7, record: 9 };
        let boat_race_2: BoatRaceData = BoatRaceData { time: 15, record: 40 };
        let boat_race_3: BoatRaceData = BoatRaceData { time: 30, record: 200 };

        let boat_races = vec![boat_race_1, boat_race_2, boat_race_3];

        let res = calculate_possible_record(boat_races);
        assert_eq!(res, vec![4,8,9])
    }

    
}
