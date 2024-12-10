use std::{collections::HashSet, fs};

type Position = (i32, i32);
#[derive(Debug, Copy, Clone)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
type Character = (Position, Direction);
type Game = (Vec<Position>, Character, Position);

fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

fn parse_input(input: &str) -> Game {
    fn char_to_direction(char: &char) -> Direction {
        match char {
            '>' => Direction::RIGHT,
            '<' => Direction::LEFT,
            'v' => Direction::DOWN,
            '^' => Direction::UP,
            _ => panic!("Invalid character"),
        }
    }

    let matrix: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (obstacles, character): (Vec<Position>, Option<Character>) = matrix
        .iter()
        .enumerate()
        .fold((vec![], None), |result, (y, row)| {
            row.iter().enumerate().fold(
                result,
                |(obstacles, character), (x, char)| match char {
                    '#' => (
                        vec![vec![(y as i32, x as i32)], obstacles].concat(),
                        character,
                    ),
                    '^' | 'v' | '<' | '>' => (
                        obstacles,
                        Some(((y as i32, x as i32), char_to_direction(char))),
                    ),
                    _ => (obstacles, character),
                },
            )
        });
    let max_y = matrix.len() as i32;
    let max_x = matrix.len() as i32;

    (obstacles, character.unwrap(), (max_y, max_x))
}

fn next_position(
    character: &Character,
    (max_y, max_x): Position,
) -> Option<Position> {
    let ((char_y, char_x), direction) = *character;
    let (next_y, next_x) = match direction {
        Direction::UP => (char_y - 1, char_x),
        Direction::RIGHT => (char_y, char_x + 1),
        Direction::DOWN => (char_y + 1, char_x),
        Direction::LEFT => (char_y, char_x - 1),
    };
    if next_y < 0 || next_x < 0 || next_y > max_y || next_x > max_x {
        None
    } else {
        Some((next_y, next_x))
    }
}

fn get_path(game: Game) -> Vec<Position> {
    let (obstacles, character, bounds) = game;
    let next_pos = next_position(&character, bounds);
    let (char_pos, direction) = character;

    if let Some(next_pos) = next_pos {
        let next_is_obstacle = obstacles.iter().any(|o_pos| *o_pos == next_pos);

        if next_is_obstacle {
            get_path((obstacles, (char_pos, next_direction(direction)), bounds))
        } else {
            let rest = get_path((obstacles, (next_pos, direction), bounds));
            vec![vec![char_pos], rest].concat()
        }
    } else {
        vec![]
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let game = parse_input(&input);
    let path = get_path(game);
    let uniques: HashSet<_> = path.iter().collect();
    println!("1: {:?}", uniques.len());
}
