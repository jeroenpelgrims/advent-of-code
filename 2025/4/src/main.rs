use std::{collections::HashSet, fs};

use itertools::{Itertools, repeat_n};

type Coordinates = (i32, i32); // (y, x)
type Grid = HashSet<Coordinates>;

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, ch)| match ch {
                '@' => Some((y as i32, x as i32)),
                _ => None,
            })
        })
        .flatten()
        .collect()
}

fn find_neighbors<'a>(
    grid: &'a Grid,
    (y, x): Coordinates,
) -> Vec<&'a Coordinates> {
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

fn can_access_paper(grid: &Grid, coords: Coordinates) -> bool {
    let neighbors = find_neighbors(grid, coords);
    grid.contains(&coords) && neighbors.len() < 4
}

fn get_accessible_papers(grid: &Grid) -> Vec<Coordinates> {
    grid.iter()
        .filter(|coordinate| can_access_paper(grid, **coordinate))
        .cloned()
        .collect()
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut grid = parse_input(&input);

    let part1 = get_accessible_papers(&grid).len();
    println!("part 1: {}", part1);

    let mut removed = 0;
    loop {
        let accessible_papers = get_accessible_papers(&grid);
        if accessible_papers.len() == 0 {
            break;
        }
        removed += accessible_papers.len();
        grid = accessible_papers.iter().fold(grid, |mut grid, coords| {
            grid.remove(coords);
            grid
        });
    }
    println!("part 2: {}", removed);
}
