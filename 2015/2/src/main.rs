use std::cmp;
use std::fs;

type Dimensions = (i32, i32, i32);
type Areas = (i32, i32, i32);

fn parse_dimensions(text: &str) -> Dimensions {
    let digits: Vec<i32> = text.split('x').map(|s| s.parse().unwrap()).collect();
    (digits[0], digits[1], digits[2])
}

fn areas((x, y, z): Dimensions) -> Areas {
    (x * y, y * z, x * z)
}

fn min_area((a1, a2, a3): Areas) -> i32 {
    cmp::min(cmp::min(a1, a2), a3)
}

fn area(text: &str) -> i32 {
    let (a1, a2, a3) = areas(parse_dimensions(text));
    let min = min_area((a1, a2, a3));
    (a1 + a2 + a3) * 2 + min
}

fn shortest_distance((x, y, z): Dimensions) -> i32 {
    let mut arr = [x, y, z];
    arr.sort();
    let [a, b, _] = arr;
    a + a + b + b
}

fn bow_distance((x, y, z): Dimensions) -> i32 {
    x * y * z
}

fn total_ribbon(value: &str) -> i32 {
    let dimension = parse_dimensions(value);
    shortest_distance(dimension) + bow_distance(dimension)
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let total = contents.lines().map(area).reduce(|a, b| a + b).unwrap();
    println!("{}", total);

    let total2 = contents
        .lines()
        .map(total_ribbon)
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{}", total2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_size_2x3x4() {
        let result = parse_dimensions("2x3x4");
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 3);
        assert_eq!(result.2, 4);
    }

    #[test]
    fn parse_size_1x1x10() {
        let result = parse_dimensions("1x1x10");
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 1);
        assert_eq!(result.2, 10);
    }

    #[test]
    fn areas_2x3x4() {
        let (a1, a2, a3) = areas(parse_dimensions("2x3x4"));
        assert_eq!(a1, 2 * 3);
        assert_eq!(a2, 3 * 4);
        assert_eq!(a3, 2 * 4);
    }

    #[test]
    fn areas_1x1x10() {
        let (a1, a2, a3) = areas(parse_dimensions("1x1x10"));
        assert_eq!(a1, 1 * 1);
        assert_eq!(a2, 1 * 10);
        assert_eq!(a3, 1 * 10);
    }

    #[test]
    fn test_min_area() {
        assert_eq!(min_area(areas(parse_dimensions("2x3x4"))), 6);
        assert_eq!(min_area(areas(parse_dimensions("1x1x10"))), 1);
    }

    #[test]
    fn test_area() {
        assert_eq!(area("2x3x4"), 58);
        assert_eq!(area("1x1x10"), 43);
    }

    #[test]
    fn test_shortest_distance() {
        assert_eq!(shortest_distance((2, 3, 4)), 10);
        assert_eq!(shortest_distance((1, 1, 10)), 4);
        assert_eq!(shortest_distance((5, 1, 4)), 1 + 1 + 4 + 4);
    }

    #[test]
    fn test_bow_distance() {
        assert_eq!(bow_distance((2, 3, 4)), 24);
        assert_eq!(bow_distance((1, 1, 10)), 10);
    }

    #[test]
    fn test_total_ribbon() {
        assert_eq!(total_ribbon("2x3x4"), 34);
        assert_eq!(total_ribbon("1x1x10"), 14);
    }
}
