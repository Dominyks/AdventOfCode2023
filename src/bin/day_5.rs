fn main() {
    // let input = file_parser::parse_file(".//input_data//day5//input_sample.txt");
    let input = file_parser::parse_file(".//input_data//day5//input.txt");

    let lines = group_data_by_new_line(&input);

    let seeds = get_seeds(&lines[0]);
    let from_to_maps = create_from_to_maps(&lines.iter().skip(1).collect());

    let smallest_location_for_seeds = smallest_location_for_seeds(&seeds, &from_to_maps);
    let smallest_location_for_ranges_of_seeds = smallest_location_for_ranges_of_seeds(&seeds, &from_to_maps);

    println!("part 1 = {}\npart 2 = {}", smallest_location_for_seeds, smallest_location_for_ranges_of_seeds);
}

fn smallest_location_for_seeds(seeds: &Vec<u32>, from_to_maps: &Vec<FromToMap>) -> u64 {
    let mut smallest_location :u64 = u64::MAX;

    for seed in seeds {
        let corresponding_location = get_location_for_seed(*seed, from_to_maps);

        reasign_if_smaller(corresponding_location, &mut smallest_location);
    }

    smallest_location
}

fn smallest_location_for_ranges_of_seeds(seeds: &Vec<u32>, from_to_maps: &Vec<FromToMap>) -> u64 {
    let mut smallest_location :u64 = u64::MAX;

    for range_of_seeds in seeds.chunks(2) {
        for seed in range_of_seeds[0]..range_of_seeds[0]+range_of_seeds[1] {
            let corresponding_location = get_location_for_seed(seed, from_to_maps);

            reasign_if_smaller(corresponding_location, &mut smallest_location);
        }
    }

    smallest_location
}

fn get_location_for_seed(seed: u32, from_to_maps: &Vec<FromToMap>) -> u64 {
    let mut corresponding_location : u64 = u64::from(seed);

    for from_to_map in from_to_maps {
        corresponding_location = compute_from_to_relation(corresponding_location, from_to_map);
    }

    corresponding_location
}

fn compute_from_to_relation(value: u64, from_to_map: &FromToMap) -> u64 {
    let mut corresponding_value = value;

    for instruction in &from_to_map.instructions {
        let from = u64::from(instruction.source_range);
        let to = u64::from(instruction.source_range) + u64::from(instruction.length) - 1;

        if from <= corresponding_value && corresponding_value <= to {
            let value_to_walk = corresponding_value - from;
            corresponding_value = u64::from(instruction.destination_range) + value_to_walk;
            break;
        }
    }

    corresponding_value
}

fn reasign_if_smaller(value: u64, value_to_compare: &mut u64) {
    if value < *value_to_compare {
        *value_to_compare = value;
    }
}

fn create_from_to_maps<'a>(data: &Vec<&Vec<&'a str>>) -> Vec<FromToMap<'a>> {
    let mut from_to_maps :Vec<FromToMap> = Vec::new();

    for map in data {
        let mut from_to_map :FromToMap = FromToMap {
            map_name : &map[0],
            instructions: Vec::new()
        };

        let iterr = map
            .iter()
            .skip(1)
            .into_iter();

        for line in iterr {
            let values :Vec<u32> = line.split(" ").into_iter().map(|x| x.parse::<u32>().unwrap()).collect();
            if values.len() == 3 {
                let instruction = Instruction {
                    destination_range: values[0],
                    source_range: values[1],
                    length: values[2],
                };

                from_to_map.instructions.push(instruction);
            }
        }

        from_to_maps.push(from_to_map);
    }

    from_to_maps
}

fn group_data_by_new_line(input: &String) -> Vec<Vec<&str>> {
    let mut blocks : Vec<Vec<&str>> = Vec::new();

    let mut vec_block : Vec<&str> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            blocks.push(vec_block.clone());
            vec_block.clear();
        } else {
            vec_block.push(line);
        }
    }
    blocks.push(vec_block.clone());

    blocks
}

fn get_seeds(seeds_vec: &Vec<&str>) -> Vec<u32> {
    let mut seeds :Vec<u32> = Vec::new();

    for seeds_line in seeds_vec {
        for seed in seeds_line.split(" ") {
            match seed.parse::<u32>() {
                Result::Ok(seed) => seeds.push(seed),
                Err(_err) => continue
            }
        }
    }

    seeds
}

#[derive(Debug)]
#[warn(dead_code)]
struct FromToMap<'a> {
    map_name: &'a str,
    instructions : Vec<Instruction>
}

#[derive(Debug)]
struct Instruction {
    destination_range: u32,
    source_range: u32,
    length: u32,
}