use itertools::Itertools;
use std::fs;

#[derive(Debug)]
struct Matrix(Vec<Vec<char>>);
type Position = (isize, isize);
type Direction = (isize, isize);

impl Matrix {
    fn new(input: &str) -> Self {
        Matrix(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn at(&self, (y, x): Position) -> char {
        self.0[y as usize][x as usize]
    }

    fn y_max(&self) -> isize {
        (self.0.len() - 1) as isize
    }

    fn x_max(&self) -> isize {
        (self.0[0].len() - 1) as isize
    }

    fn in_bounds(&self, (y, x): Position) -> bool {
        y >= 0 && x >= 0 && y <= self.y_max() && x <= self.x_max()
    }

    fn get_positions(
        &self,
        (start_y, start_x): Position,
        count: usize,
        direction: Direction,
    ) -> Option<Vec<Position>> {
        let char_cells = vec![direction; count - 1].iter().fold(
            vec![(start_y, start_x)],
            |result, (y_mod, x_mod)| {
                let (last_y, last_x) = result.last().unwrap();
                let next_element = (last_y + y_mod, last_x + x_mod);
                vec![result, vec![next_element]].concat()
            },
        );

        let all_in_bounds =
            char_cells.iter().all(|position| self.in_bounds(*position));
        if !all_in_bounds {
            None
        } else {
            Some(char_cells)
        }
    }

    fn find_word(
        &self,
        position: Position,
        word: &str,
        direction: Direction,
    ) -> bool {
        let char_positions =
            self.get_positions(position, word.len(), direction);

        if let Some(char_positions) = char_positions {
            let chars: Vec<char> = char_positions
                .iter()
                .map(|position| self.at(*position))
                .collect();
            let string = String::from_iter(chars.clone());
            string.eq(word)
        } else {
            false
        }
    }

    fn find_word_all_directions(
        &self,
        position: Position,
        word: &str,
    ) -> Vec<Direction> {
        let directions: Vec<Direction> = (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|pair| *pair != (0, 0))
            .collect();
        directions
            .iter()
            .filter_map(|direction| {
                if self.find_word(position, word, *direction) {
                    Some(*direction)
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_word_all_cells(&self, word: &str) -> usize {
        let all_positions =
            (0..=self.y_max()).cartesian_product(0..=self.x_max());
        let results: Vec<_> = all_positions
            .flat_map(|position| self.find_word_all_directions(position, word))
            .collect();
        results.len()
    }

    fn has_x_mas(&self, position: Position) -> bool {
        self.at(position) == 'A'
            && (self.find_word(position, "AM", (-1, -1))
                && self.find_word(position, "AS", (1, 1))
                || self.find_word(position, "AS", (-1, -1))
                    && self.find_word(position, "AM", (1, 1)))
            && (self.find_word(position, "AS", (-1, 1))
                && self.find_word(position, "AM", (1, -1))
                || self.find_word(position, "AM", (-1, 1))
                    && self.find_word(position, "AS", (1, -1)))
    }

    fn find_all_x_mas(&self) -> usize {
        let all_positions =
            (0..=self.y_max()).cartesian_product(0..=self.x_max());
        let results: Vec<_> = all_positions
            .filter(|position| self.has_x_mas(*position))
            .collect();
        return results.len();
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let matrix = Matrix::new(&input);
    println!("1: {:?}", matrix.find_word_all_cells("XMAS"));
    println!("2: {:?}", matrix.find_all_x_mas());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {}
}
