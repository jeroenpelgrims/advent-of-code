use std::{collections::HashMap, fs};

use itertools::{Itertools, repeat_n};

#[derive(Debug, Clone, Copy)]
enum CellContent {
    Empty,
    Paper,
}

type Coordinates = (i32, i32); // (y, x)
type Grid = HashMap<Coordinates, CellContent>;

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, ch)| {
                let content = match ch {
                    '.' => CellContent::Empty,
                    '@' => CellContent::Paper,
                    _ => panic!("Unexpected character in input"),
                };
                ((y as i32, x as i32), content)
            })
        })
        .collect()
}

fn find_neighbors<'a>(
    grid: &'a Grid,
    (y, x): Coordinates,
) -> Vec<&'a CellContent> {
    let modifiers = repeat_n(vec![-1, 0, 1], 2)
        .multi_cartesian_product()
        .map(|v| (v[0], v[1]))
        .filter(|(y_mod, x_mod)| *y_mod != 0 || *x_mod != 0)
        .collect::<Vec<_>>();
    let neighbors = modifiers.into_iter().filter_map(|(y_mod, x_mod)| {
        let mod_coords: Coordinates = (y + y_mod, x + x_mod);
        grid.get(&mod_coords)
    });
    neighbors.collect::<Vec<_>>()
}

fn can_access_paper(grid: &Grid, (y, x): Coordinates) -> bool {
    let contents = grid.get(&(y, x));
    match contents {
        None | Some(CellContent::Empty) => false,
        Some(CellContent::Paper) => {
            let paper_neighbors: i32 = find_neighbors(grid, (y, x))
                .iter()
                .map(|content| match content {
                    CellContent::Paper => 1,
                    _ => 0,
                })
                .sum();
            paper_neighbors < 4
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let grid = parse_input(&input);

    let part1 = grid
        .keys()
        .filter(|coordinate| can_access_paper(&grid, **coordinate))
        .count();
    println!("part 1: {}", part1);
}
