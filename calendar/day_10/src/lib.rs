pub fn get_answers() -> (i32, i32){
    // let input = file_parser::parse_file(".//input_data//day10//input_sample.txt");
    let input = file_parser::parse_file(".//input_data//day10//input.txt");

    let maze = parse_data(&input);

    let part_1 = part_1_ans(&maze);
    let part_2 = 0;

    (part_1, part_2)
}

fn part_1_ans(maze: &Vec<Vec<char>>) -> i32{
    let (starting_row, starting_column) = find_starting_point(&maze);
    let adjanced_tiles = get_adjanced_tiles(starting_row, starting_column, &maze);

    if adjanced_tiles.len() <= 0 {
        panic!("bad maze");
    }

    let mut next_direction :Direction = Direction::Up;
    let mut x = 0;
    let mut y = 0;

    let mut tile = 'S';
    for adjanced_tile in adjanced_tiles {
        x = adjanced_tile.0;
        y = adjanced_tile.1;

        tile = maze[x][y];

        next_direction = adjanced_tile.2;
        if let Some(x) = Maze::next_direction(&next_direction, tile) {
            x
        } else { continue; };

        // println!("tile: {}, x{} y{}", &tile, x, y);
        break;
    }

    let mut steps  = 1;
    while tile != 'S' {
        next_direction = if let Some(x) = Maze::next_direction(&next_direction, tile) {
            x
        } else { panic!()};
        (x,y) = Maze::try_walk(&next_direction, x, y, &maze).unwrap();
        tile = maze[x][y];
        // println!("tile: {}, x{} y{}", &tile, x, y);
        steps += 1;
    }


    steps / 2
}

fn get_adjanced_tiles(row: usize, column: usize, maze: &Vec<Vec<char>>) -> Vec<(usize, usize, Direction)> {
    let checks :[Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];

    let mut connected_tiles : Vec<(usize, usize, Direction)> = Vec::new();

    for direction in checks {
        let next_coordinates = Maze::try_walk(&direction, row, column, maze);

        let next_coordinates = if let Some(x) = next_coordinates {
            (x.0, x.1)
        } else { continue; };

        connected_tiles.push((next_coordinates.0, next_coordinates.1, direction));
    }

    connected_tiles
}

fn add(a: usize, b: i8) -> Option<usize> {
    if b.is_negative() {
        a.checked_sub(b.wrapping_abs() as u8 as usize)
    } else {
        a.checked_add(b as usize)
    }
}

fn parse_data(input : &String) -> Vec<Vec<char>>{
    let mut maze :Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let maze_line : Vec<char> = line.chars().collect();
        maze.push(maze_line);
    }

    maze
}

fn find_starting_point(maze :&Vec<Vec<char>>) -> (usize,usize) {
    let mut row = 0;
    let mut column = 0;

    'outer :for (i, maze_row) in maze.iter().enumerate() {
        for (y, maze_item) in maze_row.iter().enumerate() {
            if *maze_item == 'S' {
                row = i;
                column = y;
                break 'outer;
            }
        }
    }

    (row,column)
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn next_step(&self) -> (i8, i8) {
        use Direction::*;

        match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            _ => unreachable!()
        }
    }
}

struct Maze;

impl Maze {
    fn try_walk(direction: &Direction, current_row: usize, current_column: usize, maze: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        let (n_row, n_column) = direction.next_step();

        let n_row = add(current_row, n_row);
        let n_column = add(current_column, n_column);

        if n_row.is_none() || n_column.is_none() {
            return None;
        }

        let n_row = n_row.unwrap();
        let n_column = n_column.unwrap();

        let n_row = if let Some(x) = maze.get(n_row) {
            n_row
        } else { return None };

        let n_column = if let Some(x) = maze[n_row].get(n_column) {
            n_column
        } else { return None };

        return Some((n_row, n_column))
    }

    fn next_direction(last_direction: &Direction, current_tile: char) -> Option<Direction> {
        match last_direction {
            Direction::Up => {
                match current_tile {
                    '|' => Some(Direction::Up),
                    '7' => Some(Direction::Left),
                    'F' => Some(Direction::Right),
                    _ => None
                }
            }
            Direction::Down => {
                match current_tile {
                    '|' => Some(Direction::Down),
                    'L' => Some(Direction::Right),
                    'J' => Some(Direction::Left),
                    _ => None
                }
            }
            Direction::Left => {
                match current_tile {
                    '-' => Some(Direction::Left),
                    'L' => Some(Direction::Up),
                    'F' => Some(Direction::Down),
                    _ => None
                }
            }
            Direction::Right => {
                match current_tile {
                    '-' => Some(Direction::Right),
                    'J' => Some(Direction::Up),
                    '7' => Some(Direction::Down),
                    _ => None
                }
            }
        }
    }
}
