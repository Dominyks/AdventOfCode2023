use std::ops::Neg;
use file_parser;

fn main() {
    // let input = file_parser::parse_file(".//input_data//day9//input_sample.txt");
    let input = file_parser::parse_file(".//input_data//day9//input.txt");

    let parsed_data = parse_data(&input);

    let part_1 = get_part_1_ans(&parsed_data);
    let part_2 = get_part_2_ans(&parsed_data);

    println!("part 1 = {}\npart 2 = {}", part_1, part_2);
}

fn get_part_1_ans(vecs: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for vec in vecs {
        sum += calculate_next_sequence_number(vec);
    }

    sum
}

fn get_part_2_ans(vecs: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for vec in vecs {
        sum += calculate_next_sequence_number2(&vec);
    }

    sum
}

fn calculate_next_sequence_number2(nums: &Vec<i32>) -> i32 {
    let mut history_rows : Vec<Vec<i32>> = Vec::new();
    history_rows.push(nums.to_vec());

    let mut count = history_rows.len();
    let mut current_index = 0;

    while !history_rows[count - 1].iter().all(|&x| x == 0)
    {
        history_rows.push(get_next_history_row(&history_rows[current_index]));
        count = history_rows.len();
        current_index += 1;
    }

    let mut current = 0;
    for i in (0..history_rows.len() - 1).rev() {

        let mut curr = &mut history_rows[i];
        let val = curr[0];
        current = val + current.neg();
    }

    for history_row in history_rows {

        let mut stringg = "".to_string();

        for history_row_num in history_row {
            stringg += &(history_row_num.to_string() + " ")
        }
    }

    current
}

fn calculate_next_sequence_number(nums: &Vec<i32>) -> i32 {
    let mut history_rows : Vec<Vec<i32>> = Vec::new();
    history_rows.push(nums.to_vec());

    let mut count = history_rows.len();
    let mut current_index = 0;

    while !history_rows[count - 1].iter().all(|&x| x == 0)
    {
        history_rows.push(get_next_history_row(&history_rows[current_index]));
        count = history_rows.len();
        current_index += 1;
    }

    let mut current = 0;
    for i in (0..history_rows.len() - 1).rev() {
        let count = history_rows[i].len();

        current += history_rows[i][count -1];
        history_rows[i].push(current);
    }

    for history_row in history_rows {
        let mut stringg = "".to_string();

        for history_row_num in history_row {
            stringg += &(history_row_num.to_string() + " ")
        }
    }

    current
}

fn get_next_history_row(row: &Vec<i32>) -> Vec<i32> {
    let mut history_row: Vec<i32> = Vec::new();
    for i in 0..row.len() - 1 {
        let difference = row[i+1] - row[i];
        history_row.push(difference);
    }

    history_row
}

fn parse_data(input : &String) -> Vec<Vec<i32>> {
    let mut vecs : Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let nums :Vec<i32> = line.split(' ').into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        vecs.push(nums);
    }

    vecs
}