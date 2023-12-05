use file_parser;
use day_1;
use day_2;
use day_3;
use day_4;
use day_5;

fn main() {
    // let input = file_parser::parse_file(".//input_data//day5//input_sample.txt");
    let input = file_parser::parse_file(".//input_data//day5//input.txt");
    let lines = input.lines();

    // let day_2_ans = day_1::part_1(&mut lines.clone());
    // let day_1_ans = day_1::part_2(&mut lines.clone());

    let (part_1, part_2) = day_5::get_answers();

    println!("part 1 = {:?}\npart 2 = {:?}", part_1, part_2);
}