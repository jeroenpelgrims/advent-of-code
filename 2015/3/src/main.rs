use std::fs;

type Point = (i32, i32);

fn coordinates(path: &str) -> Vec<Point> {
    let coords = path.chars().fold(vec![(0, 0)], |mut result, item| {
        let last = result.last().unwrap();
        let x = last.0;
        let y = last.1;
        let next = match item {
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            _ => (0, 0),
        };
        result.push(next);
        result
    });
    coords
}

fn count_unique(coords: Vec<Point>) -> usize {
    let mut clone = coords.clone();
    clone.sort();
    clone.dedup();
    clone.len()
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let coords = coordinates(&input);
    println!("{}", count_unique(coords))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates() {
        assert_eq!(coordinates(">"), vec![(0, 0), (1, 0)]);
    }

    #[test]
    fn test_coordinates2() {
        assert_eq!(
            coordinates("^>v<"),
            vec![(0, 0), (0, 1), (1, 1), (1, 0), (0, 0),]
        );
    }

    #[test]
    fn test_coordinates3() {
        assert_eq!(
            coordinates("^v^v^v^v^v"),
            vec![
                (0, 0),
                (0, 1),
                (0, 0),
                (0, 1),
                (0, 0),
                (0, 1),
                (0, 0),
                (0, 1),
                (0, 0),
                (0, 1),
                (0, 0),
            ]
        );
    }

    #[test]
    fn test_count_unique() {
        assert_eq!(1, count_unique(vec![(0, 0), (0, 0), (0, 0),]));
        assert_eq!(2, count_unique(vec![(0, 0), (1, 0), (0, 0),]));
    }
}
