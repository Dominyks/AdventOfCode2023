use std::str::Lines;

static RADIX: u32 = 10;

pub fn part_1(input : &mut Lines) -> u32 {
    let mut sum = 0;

    for line in input {
        let mut number_vec :Vec<u32> = Vec::new();

        for char in line.chars() {
            if char.is_numeric() {
                let num = char.to_digit(RADIX).unwrap();
                number_vec.push(num);
            }
        }

        if number_vec.len() > 0 {
            sum += combine_first_and_last_digits(&number_vec);
        }
    }

    sum
}

pub fn part_2(input : &mut Lines) -> u32 {
    let mut sum = 0;

    for line in input.by_ref() {

        let mut number_vec :Vec<u32> = Vec::new();
        let mut letters = String::new();

        for char in line.chars() {
            if char.is_numeric() {
                let num = char.to_digit(RADIX).unwrap();
                number_vec.push(num);
                continue;
            }

            letters.push(char);
            let letter_number = is_numeric_string(&letters);

            if letter_number != 0 {
                number_vec.push(letter_number);
            }
        }

        if number_vec.len() != 0 {
            sum += combine_first_and_last_digits(&number_vec);
        }
    }

    sum
}

fn is_numeric_string(letter: &str) -> u32 {
    if letter.ends_with("one") { return 1 }
    if letter.ends_with("two") { return 2 }
    if letter.ends_with("three") { return 3 }
    if letter.ends_with("four") { return 4 }
    if letter.ends_with("five") { return 5 }
    if letter.ends_with("six") { return 6 }
    if letter.ends_with("seven") { return 7 }
    if letter.ends_with("eight") { return 8 }
    if letter.ends_with("nine") { return 9 }

    return 0
}

fn combine_first_and_last_digits(numbers : &Vec<u32>) -> u32 {
    let first = numbers[0];
    let last = numbers[numbers.len()-1];

    first * 10 + last
}