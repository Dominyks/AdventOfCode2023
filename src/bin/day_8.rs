use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    // let input = file_parser::parse_file(".//input_data//day8//input_sample.txt");
    let input = file_parser::parse_file(".//input_data//day8//input.txt");

    let (directions, maps) = parse_input(&input);

    let part_1 = get_how_many_steps_to_reach_end(&directions, &maps);
    let part_2 = get_how_many_steps_to_reach_end_multiple_paths(&directions, &maps);

    println!("part 1 = {}\npart 2 = {}", part_1, part_2);
}

fn get_how_many_steps_to_reach_end_multiple_paths(directions: &Vec<char>, maps: &HashMap<&str, Map>) -> i64 {
    let mut steps:i64 = 0;
    const START_LOCATION_ENDING: char = 'A';
    const END_LOCATION_ENDING: char = 'Z';

    let mut current_locations: Vec<&Map> = Vec::new();
    for location in maps.keys() {
        if location.ends_with(START_LOCATION_ENDING) {
            current_locations.push(&maps[location])
        }
    }

    let count = current_locations.len();

    let steps_to_find_z:Vec<_> = current_locations.iter().map(|mut x| get_z_find_step_count(x, directions, maps)).collect();
    let llcms = steps_to_find_z.into_iter().fold(1, |a, b| lcm(a, b));

    llcms
}

fn gcd(mut a: i64,mut  b: i64) -> i64 {
    // Euclidean algorithm
    while (b != 0) {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b / gcd(a, b))
}

fn get_z_find_step_count<'a>(mut current_location: &'a Map<'a>, directions: &Vec<char>, maps: &'a HashMap<&str, Map>) -> i64 {
    const START_LOCATION_ENDING: char = 'A';
    const END_LOCATION_ENDING: char = 'Z';

    let mut steps = 0;
    let mut walk = true;
    while walk {
        for direction in directions {
            current_location = get_next_location(direction, &maps, current_location);

            steps += 1;
            if current_location.location.ends_with(END_LOCATION_ENDING)  {
                println!("{} {}", steps, current_location.location);
                walk = false;
                break;
            }
        }
    }

    steps
}

fn get_next_location<'a>(direction: &char, maps: &'a HashMap<&str, Map>, current_location: &Map) -> &'a Map<'a> {
    match *direction {
        'L' => &maps[current_location.left_destination],
        'R' => &maps[current_location.right_destination],
        _ => panic!("bad direction")
    }
}

fn get_how_many_steps_to_reach_end(directions: &Vec<char>, maps: &HashMap<&str, Map>) -> i32 {
    let mut steps = 0;
    const START_MARKER: &str = "AAA";
    const END_MARKER: &str = "ZZZ";

    let mut current_location = &maps[START_MARKER];

    let mut walk = true;
    while walk {
        for direction in directions {
            current_location = get_next_location(direction, &maps, current_location);

            steps += 1;
            if current_location.location == END_MARKER {
                walk = false;
                break;
            }

        }
    }

    steps
}

fn parse_input<'a>(input: &'a String) -> (Vec<char>, HashMap<&'a str, Map<'a>>) {
    let mut directions : Vec<char> = Vec::new();
    let mut maps : HashMap<&'a str, Map<'a>> = HashMap::new();

    let mut lines = input.lines().into_iter();
    directions = lines.next().unwrap().chars().collect();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut line = line.split('=').into_iter();
        let location = line.next().unwrap().trim();

        for split_part in line {
            let mut split_part = split_part.trim().split(", ").into_iter();

            let left = split_part.next().unwrap().trim_start_matches('(');

            let right = split_part.next().unwrap().trim_end_matches(')');

            let map = Map {
                location,
                left_destination: left,
                right_destination: right
            };

            maps.insert(location, map);
        }
    }

    (directions, maps)
}

#[derive(Debug)]
struct Map<'a> {
    location: &'a str,
    left_destination: &'a str,
    right_destination: &'a str
}