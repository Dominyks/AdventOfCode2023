use std::str::Lines;
static REDS: u32 = 12;
static GREENS: u32 = 13;
static BLUES: u32 = 14;

fn main() {
    let input = file_parser::parse_file(".//input_data//day2//input.txt");
    let lines = input.lines();

    let part_1 = part_1(&mut lines.clone());
    let part_2 = part_2(&mut lines.clone());

    println!("part 1 = {}\npart 2 = {}", part_1, part_2);
}

pub fn part_1(lines : &mut Lines) -> u32 {

    let mut game: Vec<(u32, Vec<Vec<GameSet>>)> = Vec::new();

    for line in lines {
        let game_line = line.split(":").collect::<Vec<&str>>();

        if game_line.len() == 2 {
            let id = get_line_id(&game_line[0]);
            // dbg!(id);

            let game_sets_data = game_line[1].split(";");
            let mut game_sets : Vec<Vec<GameSet>> = Vec::new();

            for set_hand in game_sets_data {
                // dbg!(set_hand);

                let mut game_hands : Vec<GameSet> = Vec::new();

                let set_combos = set_hand.trim().split(",");
                for set_combo in set_combos {
                    let combination = set_combo.trim().split(" ").collect::<Vec<&str>>();

                    let game_set = GameSet {number: combination[0].parse().unwrap(), color: combination[1].to_string()};
                    game_hands.push(game_set);
                    // dbg!(game_set);
                }

                game_sets.push(game_hands);
            }

            game.push((id, game_sets));

        }
    }

    let mut sum_false_ids = 0;


    for game_line in game {
        let mut is_possible = true;

        for game_hands in game_line.1 {
            for game_set in game_hands {
                let more_than = match game_set.color.as_str() {
                    "red" => game_set.number > REDS,
                    "green" => game_set.number > GREENS,
                    "blue" => game_set.number > BLUES,
                    _ => false
                };


                if more_than {
                    // dbg!(game_set);
                    is_possible = false;
                    break;
                }
            }
        }

        if is_possible {
            sum_false_ids += game_line.0;
        }
    }

    sum_false_ids
}

pub fn part_2(lines : &mut Lines) -> u32 {
    let mut game: Vec<(u32, Vec<Vec<GameSet>>)> = Vec::new();

    for line in lines {
        let game_line = line.split(":").collect::<Vec<&str>>();

        if game_line.len() == 2 {
            let id = get_line_id(&game_line[0]);
            // dbg!(id);

            let game_sets_data = game_line[1].split(";");
            let mut game_sets : Vec<Vec<GameSet>> = Vec::new();

            for set_hand in game_sets_data {
                // dbg!(set_hand);

                let mut game_hands : Vec<GameSet> = Vec::new();

                let set_combos = set_hand.trim().split(",");
                for set_combo in set_combos {
                    let combination = set_combo.trim().split(" ").collect::<Vec<&str>>();

                    let game_set = GameSet {number: combination[0].parse().unwrap(), color: combination[1].to_string()};
                    game_hands.push(game_set);
                    // dbg!(game_set);
                }

                game_sets.push(game_hands);
            }

            game.push((id, game_sets));

        }
    }

    let mut sum = 0;

    for game_line in game {
        let mut biggest_red = 0;
        let mut biggest_green = 0;
        let mut biggest_blue = 0;

        for game_hands in game_line.1 {
            for game_set in game_hands {
                match game_set.color.as_str() {
                    "red" => {
                        if game_set.number > biggest_red { biggest_red = game_set.number }
                    },
                    "green" => {
                        if game_set.number > biggest_green { biggest_green = game_set.number }
                    },
                    "blue" =>  {
                        if game_set.number > biggest_blue { biggest_blue = game_set.number }
                    },
                    _ => ()
                };

                dbg!(biggest_red);
                dbg!(biggest_green);
                dbg!(biggest_blue);
                dbg!("*****");
            }
        }

        sum += biggest_red * biggest_green * biggest_blue;
    }

    sum
}

fn get_line_id(game_line: &str) -> u32 {
    game_line.split(" ").nth(1).unwrap().parse().unwrap()
}

#[derive(Debug)]
struct GameSet {
    number: u32,
    color: String
}