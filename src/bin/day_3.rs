use std::str::Lines;

fn main() {
    let input = file_parser::parse_file(".//input_data//day3//input.txt");
    let lines = input.lines();

    let part_1 = part_1(&mut lines.clone());
    let part_2 = part_2(&mut lines.clone());

    println!("part 1 = {}\npart 2 = {}", part_1, part_2);
}
pub fn part_1(lines: &mut Lines) -> u32 {
    let mut dataset: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line.chars().collect::<Vec<char>>();
        dataset.push(line);
    }

    let height: usize = dataset.len();
    let width: usize = dataset[0].len();

    let mut nums_and_indexes: Vec<(u32, Vec<(usize, usize)>)> = Vec::new();

    let mut sum = 0;

    for row in 0..height {
        let mut number_chars: String = "".to_string();
        let mut indexes: Vec<(usize, usize)> = Vec::new();

        for column in 0..width {
            if dataset[row][column].is_digit(10) {
                number_chars.push(dataset[row][column]);
                indexes.push((row, column));
            } else {
                if number_chars.len() > 0 {
                    nums_and_indexes.push((number_chars.parse::<u32>().unwrap(), indexes.clone()));
                }

                number_chars.clear();
                indexes.clear();
                continue;
            }
        }

        if number_chars.len() > 0 {
            nums_and_indexes.push((number_chars.parse::<u32>().unwrap(), indexes.clone()));
        }
    }

    for (num, index_arr) in &nums_and_indexes {
        for index in index_arr {
            if is_adjacent(&dataset, index.0, index.1, height, width, is_any_symbol).0 {
                sum += num;
                break;
            }
        }
    }


    sum
}

#[warn(unused_comparisons)]
fn is_valid_pos(row_i: usize, column_i: usize, arr_length: usize, arr_width: usize) -> bool {
    if row_i < 0 || column_i < 0 || row_i > (arr_length - 1).try_into().unwrap() || column_i > (arr_width - 1).try_into().unwrap() {
        return false;
    }

    true
}

fn is_any_symbol(array_2d: &Vec<Vec<char>>, check_row_i: usize, check_column_i: usize) -> bool {
    let x: char = array_2d[check_row_i][check_column_i];
    if !x.is_digit(10) && x != '.' {
        return true;
    }

    false
}

fn is_adjacent<'a>(
    array_2d: &'a Vec<Vec<char>>,
    row_i: usize,
    column_i: usize,
    height: usize,
    length: usize,
    f_for_symbol: fn(&'a Vec<Vec<char>>, usize, usize) -> bool,
) -> (bool, Vec<(char, (usize, usize))>) {
    let mut adjacent_symbols: Vec<(char, (usize, usize))> = Vec::new();

    if try_sub(row_i, 1) && try_sub(column_i, 1) {
        let new_row = row_i - 1;
        let new_column = column_i - 1;
        if is_valid_pos(new_row, new_column, height, length) &&
            f_for_symbol(&array_2d, new_row, new_column) {
            // return true;
            adjacent_symbols.push((array_2d[new_row][new_column].clone(), (new_row, new_column)));
        }

        let new_column = column_i;
        if is_valid_pos(new_row, new_column, height, length) &&
            f_for_symbol(&array_2d, new_row, new_column) {
            // return true;
            adjacent_symbols.push((array_2d[new_row][new_column].clone(), (new_row, new_column)));
        }

        let new_column = column_i + 1;
        if is_valid_pos(new_row, new_column, height, length) &&
            f_for_symbol(&array_2d, new_row, new_column) {
            // return true;
            adjacent_symbols.push((array_2d[new_row][new_column].clone(), (new_row, new_column)));
        }
    }
    ///////////////////////////

    if try_sub(column_i, 1) {
        let new_row = row_i;
        let new_column = column_i - 1;
        if is_valid_pos(new_row, new_column, height, length) &&
            f_for_symbol(&array_2d, new_row, new_column) {
            // return true;
            adjacent_symbols.push((array_2d[new_row][new_column].clone(), (new_row, new_column)));
        }

        let new_column = column_i + 1;
        if is_valid_pos(new_row, new_column, height, length) &&
            f_for_symbol(&array_2d, new_row, new_column) {
            // return true;
            adjacent_symbols.push((array_2d[new_row][new_column].clone(), (new_row, new_column)));
        }
    }

    ///////////////////
    if try_sub(column_i, 1) {
        let new_row = row_i + 1;
        let new_column = column_i - 1;
        if is_valid_pos(new_row, new_column, height, length) &&
            f_for_symbol(&array_2d, new_row, new_column) {
            // return true;
            adjacent_symbols.push((array_2d[new_row][new_column].clone(), (new_row, new_column)));
        }

        let new_column = column_i;
        if is_valid_pos(new_row, new_column, height, length) &&
            f_for_symbol(&array_2d, new_row, new_column) {
            // return true;
            adjacent_symbols.push((array_2d[new_row][new_column].clone(), (new_row, new_column)));
        }

        let new_column = column_i + 1;
        if is_valid_pos(new_row, new_column, height, length) &&
            f_for_symbol(&array_2d, new_row, new_column) {
            // return true;
            adjacent_symbols.push((array_2d[new_row][new_column].clone(), (new_row, new_column)));
        }
    }

    if adjacent_symbols.len() > 0 {
        return (true, adjacent_symbols);
    }

    (false, adjacent_symbols)
}

fn try_sub(x: usize, a: usize) -> bool {
    if a < x + a {
        return true;
    }
    false
}

pub fn part_2(lines: &mut Lines) -> u32 {
    let mut dataset: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line.chars().collect::<Vec<char>>();
        dataset.push(line);
    }

    let height: usize = dataset.len();
    let width: usize = dataset[0].len();

    let mut nums_and_indexes: Vec<(u32, Vec<(usize, usize)>)> = Vec::new();
    let mut gear_indexes: Vec<(usize, usize)> = Vec::new();

    let mut sum = 0;

    for row in 0..height {
        let mut number_chars: String = "".to_string();
        let mut indexes: Vec<(usize, usize)> = Vec::new();

        for column in 0..width {
            if dataset[row][column].is_digit(10) {
                number_chars.push(dataset[row][column]);
                indexes.push((row, column));
            } else {
                if dataset[row][column] == '*' {
                    gear_indexes.push((row, column))
                }

                if number_chars.len() > 0 {
                    nums_and_indexes.push((number_chars.parse::<u32>().unwrap(), indexes.clone()));
                }

                number_chars.clear();
                indexes.clear();
                continue;
            }
        }

        if number_chars.len() > 0 {
            nums_and_indexes.push((number_chars.parse::<u32>().unwrap(), indexes.clone()));
        }
    }

    for (row, column) in gear_indexes {
        let mut gear_parts: Vec<&u32> = Vec::new();

        let (is_adjanced, adjanced_pos) = is_adjacent(&dataset, row, column, height, width, is_number);

        if is_adjanced {
            for (_, pos) in adjanced_pos {
                for (num, indexes) in &nums_and_indexes {
                    for index in indexes {
                        if index.0 == pos.0 && index.1 == pos.1 {
                            if !gear_parts.contains(&num) {
                                gear_parts.push(&num);
                            };
                            break;
                        }
                    }
                }
            }
        }

        if gear_parts.len() == 2 {
            sum += gear_parts[0] * gear_parts[1];
        }
    }

    sum
}

fn is_number(array_2d: &Vec<Vec<char>>, check_row_i: usize, check_column_i: usize) -> bool {
    let x: char = array_2d[check_row_i][check_column_i];
    if x.is_digit(10) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
