use file_parser;
use day_1;
use day_2;

fn main() {
    // let input = file_parser::parse_file(".//input_data//day2//input_sample.txt");
    let input = file_parser::parse_file(".//input_data//day2//input.txt");
    let mut lines = input.lines();

    // let day_2_ans = day_1::part_1(&mut lines.clone());
    // let day_1_ans = day_1::part_2(&mut lines.clone());

    let day_1_ans = day_2::part_1(&mut lines.clone());
    let day_2_ans = day_2::part_2(&mut lines.clone());

    println!("{:?}, {:?}", day_1_ans, day_2_ans);
}