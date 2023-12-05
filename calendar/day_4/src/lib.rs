use std::str::Lines;

pub fn part_1(lines: &mut Lines) -> u32 {
    let prepared_data = prepare_data(lines);

    let mut sum = 0;

    for card_ticket in prepared_data {
        let mut points = 0;
        for winning_number in card_ticket.winning_numbers {
            if card_ticket.current_numbers.contains(&winning_number) {
                if points == 0 {
                    points+=1;
                } else {
                    points *=2;
                }
            }
        }

        sum += points;
    }

    sum
}

pub fn part_2(lines: &mut Lines) -> u32 {
    let mut prepared_data = prepare_data(lines);

    // println!(
    //     "{:?}",
    //     (1..5).map(|i| (i + i, i * i)).collect::<HashMap<_, _>>()
    // );

    // let hash_map :HashMap<u32, CardTicket> = prepared_data.map(|x| (x.id, x)).collect();

    for i in 0..prepared_data.len() {
        let mut cards_won = 0;

        for winning_number in &prepared_data[i].winning_numbers {
            if prepared_data[i].current_numbers.contains(winning_number) {
                cards_won += 1;
            }
        }

        let from = i+1;
        let to = i+cards_won;

        for x in from..=to {
            let how_much_to_add = prepared_data[i].amount;
            prepared_data[x].amount += how_much_to_add;
        }
    }

    prepared_data.iter().map(|y| y.amount).sum()
}

fn prepare_data(lines: &mut Lines) -> Vec<CardTicket> {
    let mut dataset: Vec<CardTicket> = Vec::new();

    for line in lines {
        let mut card_ticket: CardTicket = CardTicket {
            id: 0,
            winning_numbers: vec![],
            current_numbers: vec![],
            amount: 1,
        };

        let game_line = line.split(":").collect::<Vec<&str>>();

        if game_line.len() == 2 {
            card_ticket.id = get_line_id(&game_line[0]);

            let mut numbers = game_line[1].split('|');

            let a: String = numbers.by_ref().take(1).collect::<String>();
            let winning_number_vec: Vec<&str> = a.trim().split(' ').collect();
            card_ticket.winning_numbers = winning_number_vec
                .iter()
                .filter(|x| x != &&"")
                .map(|x| x.parse::<u32>().unwrap()).collect(); // where ok()

            let b: String = numbers.by_ref().take(1).collect::<String>();
            let current_numbers_vec: Vec<&str> = b.trim().split(' ').collect();
            card_ticket.current_numbers = current_numbers_vec
                .iter()
                .filter(|x| x != &&"")
                .map(|x| x.parse::<u32>().unwrap()).collect(); // where ok()
        }

        dataset.push(card_ticket);
    }

    dataset
}

fn get_line_id(game_line: &str) -> u32 {
    game_line.split(" ").last().unwrap().parse().unwrap()
}


#[derive(Debug)]
struct CardTicket {
    id: u32,
    winning_numbers: Vec<u32>,
    current_numbers: Vec<u32>,
    amount: u32,
}