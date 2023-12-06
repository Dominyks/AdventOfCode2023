use std::ops::Add;
use file_parser;

pub fn get_answers() -> (i64, i64) {
    // let input = file_parser::parse_file(".//input_data//day6//input_sample.txt");
    let input = file_parser::parse_file(".//input_data//day6//input.txt");

    let (times, distances) = get_race_times_and_distance_arrays(&input);

    let way_to_win_all_races = get_num_of_ways_to_win_all_races(&times, &distances);
    let way_to_win_single_total_race = get_num_of_ways_to_win_single_race(&times, &distances);

    (way_to_win_all_races, way_to_win_single_total_race)
}

fn get_num_of_ways_to_win_all_races(times: &Vec<i64>, distances: &Vec<i64>) -> i64 {
    let mut possible_ways_to_beat_time : Vec<i64> = Vec::new();
    for index in 0..times.len() {
        let num_of_ways_to_win = get_ways_to_beat_time(times[index], distances[index]);
        possible_ways_to_beat_time.push(num_of_ways_to_win);
    }

    let mut ways_to_win_all_races :i64 = 1;
    for num in possible_ways_to_beat_time {
        ways_to_win_all_races *= num;
    }

    ways_to_win_all_races
}

fn get_ways_to_beat_time(time: i64, distance_to_beat: i64) -> i64 {
    let mut num_of_ways_to_win = 0;

    for mili_sec in 0..time {
        let remaining_time = time - mili_sec;
        let speed : i64 = mili_sec * 1;
        let distance_traveled = speed * remaining_time;

        if distance_traveled > distance_to_beat {
            num_of_ways_to_win += 1;
        }
    }

    num_of_ways_to_win
}

fn get_num_of_ways_to_win_single_race(times: &Vec<i64>, distances: &Vec<i64>) -> i64 {
    let time: String = times.iter().map( |id| id.to_string()).collect();
    let distance: String = distances.iter().map( |id| id.to_string()).collect();

    let ways_to_beat = get_ways_to_beat_time(time.parse::<i64>().unwrap(), distance.parse::<i64>().unwrap());

    ways_to_beat
}

fn get_race_times_and_distance_arrays(input: &String) -> (Vec<i64>, Vec<i64>){
    let mut lines = input.lines();

    let times = lines.next().unwrap();
    let times: Vec<i64> = times
        .split(" ")
        .filter_map(|str| str.parse::<i64>().ok())
        .collect();

    let distances = lines.next().unwrap();
    let distances: Vec<i64> = distances
        .split(" ")
        .filter_map(|str| str.parse::<i64>().ok())
        .collect();

    if times.len() != distances.len() {
        panic!("bad input data");
    }

    (times, distances)
}