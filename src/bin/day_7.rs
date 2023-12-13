
fn main() {
    // let input = file_parser::parse_file(".//input_data//day7//input_sample.txt");
    let input = file_parser::parse_file(".//input_data//day7//input.txt");

    let hands = parse_hands(input);

    let part_1 = get_value_part_1(hands.clone());
    let part_2 = get_value_part_2(hands.clone());

    println!("part 1 = {}\npart 2 = {}", part_1, part_2);
}

fn get_value_part_1(mut hands: Vec<Hand>) -> usize {
    hands.sort_by(|a,b| if a.get_type() == b.get_type() {
        a.get_hand_num().partial_cmp(&b.get_hand_num()).unwrap()
    } else {
        a.get_type().partial_cmp(&b.get_type()).unwrap()
    });

    let mut sum : usize = 0;
    for (i, hand) in hands.iter().enumerate() {
        let value = hand.bid * (i+1);
        sum += value;
    }


    sum
}

fn get_value_part_2(mut hands: Vec<Hand>) -> usize {
    hands.sort_by(|a,b| if a.get_type_with_joker() == b.get_type_with_joker() {
        a.get_hand_num_with_joker().partial_cmp(&b.get_hand_num_with_joker()).unwrap()
    } else {
        a.get_type_with_joker().partial_cmp(&b.get_type_with_joker()).unwrap()
    });

    let mut sum : usize = 0;
    for (i, hand) in hands.iter().enumerate() {

        let value = hand.bid * (i+1);
        sum += value;
    }

    sum
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [char; 5],
    bid: usize,
}

impl Hand {
    fn get_hand_num(&self) -> i32 {
        let mut value : usize = 0;

        for i in 0..5 {
            let slice_indice: usize = 4;
            let card = Self::get_card_value(self.cards[i]);
            let raise = usize::pow(16, (slice_indice-i).try_into().unwrap());
            value += card * raise;
        }

        value.try_into().unwrap()
    }

    fn get_hand_num_with_joker(&self) -> i32 {
        let mut value : usize = 0;

        for i in 0..5 {
            let slice_indice: usize = 4;
            let card = Self::get_card_value_with_joker(self.cards[i]);
            let raise = usize::pow(16, (slice_indice-i).try_into().unwrap());
            value += card * raise;
        }

        value.try_into().unwrap()
    }

    fn get_card_value(char: char) -> usize {
        match char {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => 0
        }
    }

    fn get_card_value_with_joker(char: char) -> usize {
        match char {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => 0
        }
    }

    fn get_type(&self) -> Type {
        let mut values :Vec<(char, u8)> = Vec::new();

        for i in 0..5 {
            let value = &self.cards[i];
            if values.iter().find(|x| x.0 == *value).is_some() {
                continue;
            };

            let mut count = 0;
            for card in &self.cards {
                if card == value {
                    count += 1;
                }
            }
            values.push((*value, count));
        }

        values.sort_by(|a, b| b.1.cmp(&a.1));

        get_hand_strength(&values)
    }

    fn get_type_with_joker(&self) -> Type {
        let mut values :Vec<(char, u8)> = Vec::new();

        let mut joker_count = 0;

        for i in 0..5 {
            let value = &self.cards[i];

            if *value == 'J' {
                joker_count += 1;
            }

            if values.iter().find(|x| x.0 == *value).is_some() {
                continue;
            };

            let mut count = 0;
            for card in &self.cards {

                if card == value {
                    count += 1;
                }
            }
            values.push((*value, count));
        }

        values.sort_by(|a, b| b.1.cmp(&a.1));

        if joker_count == 0 {
            return get_hand_strength(&values);
        }

        let unique_cards_count = values.iter().count();

        let strongest_hand_num = 0;
        let mut strongest_hand_type = Type::HighCard;

        for i in 0..unique_cards_count {
            let mut new_values = values.clone();
            let mut new_values_without_j :Vec<(char, u8)> = Vec::new();

            if new_values[i].0 != 'J' {
                new_values[i].1 += joker_count;

                for new_value in new_values {
                    if new_value.0 == 'J' { continue; }
                    new_values_without_j.push(new_value);
                }

                new_values_without_j.sort_by(|a, b| b.1.cmp(&a.1));
            } else {
                new_values_without_j = new_values;
            }

            let hand_strength = get_hand_strength(&new_values_without_j);

            if hand_strength > strongest_hand_type {
                strongest_hand_type = hand_strength;
            }
        }


        strongest_hand_type
    }
}

fn get_hand_strength(values : &Vec<(char, u8)>) -> Type {
    let unique_cards_count = values.iter().count();

    if unique_cards_count == 1 {
        return Type::FiveOfAKind;
    } else if unique_cards_count == 2 {
        if values[0].1 == 4 {
            return Type::FourOfAKind;
        } else if values[0].1 == 3 && values[1].1 == 2 {
            return Type::FullHouse;
        }
    } else if unique_cards_count == 3 {
        if values[0].1 == 3 {
            return Type::ThreeOfAKind;
        } else if values[0].1 == 2 && values[1].1 == 2 {
            return Type::TwoPair;
        }
    } else if unique_cards_count == 4 {
        return Type::OnePair;
    }

    Type::HighCard
}

fn parse_hands(input: String) -> Vec<Hand> {
    let mut hands :Vec<Hand> = Vec::new();

    for line in input.lines() {

        let mut line = line.split(" ");
        let cards = line.next().expect("bad input");
        let bid = line.next().expect("bad input").parse::<usize>().expect("bad inout");

        let mut cards_arr: [char; 5] = ['_'; 5];
        for (i, char) in cards.chars().enumerate() {
            cards_arr[i] = char;
        }

        hands.push(Hand {
            bid: bid,
            cards: cards_arr
        })
    }

    hands
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}


