use std::fs;

type ProductId = u64;
type FreshRange = (ProductId, ProductId);

fn is_fresh(product_id: &ProductId, fresh_ranges: &Vec<FreshRange>) -> bool {
    fresh_ranges.iter().fold(false, |fresh, (min, max)| {
        fresh || *product_id >= *min && *product_id <= *max
    })
}

fn parse_input(input: &str) -> (Vec<FreshRange>, Vec<ProductId>) {
    let Some((ranges_str, product_ids_str)) = input.split_once("\n\n") else {
        panic!("Input format can't be parsed");
    };

    let product_ids: Vec<ProductId> = product_ids_str
        .trim()
        .lines()
        .map(|line| {
            line.parse::<ProductId>()
                .expect(&format!("Can't parse line: {}", line))
        })
        .collect::<Vec<_>>();
    let fresh_ranges: Vec<FreshRange> = ranges_str
        .lines()
        .filter_map(|line| {
            line.split_once("-").map(|(min, max)| {
                (
                    min.parse::<ProductId>()
                        .expect(&format!("Can't parse min: {}", min)),
                    max.parse::<ProductId>()
                        .expect(&format!("Can't parse max: {}", max)),
                )
            })
        })
        .collect::<Vec<_>>();

    (fresh_ranges, product_ids)
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (fresh_ranges, product_ids) = parse_input(&input);
    let part1 = product_ids
        .into_iter()
        .filter(|id| is_fresh(id, &fresh_ranges))
        .count();
    println!("part1: {}", part1)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_fresh_tests {
        use std::fs;

        use super::*;

        #[test]
        fn test_is_fresh() {
            let input = fs::read_to_string("./test-input.txt")
                .expect("Can't read input file");
            let (fresh_ranges, _) = parse_input(&input);

            assert!(!is_fresh(&1, &fresh_ranges));
            assert!(is_fresh(&5, &fresh_ranges));
            assert!(!is_fresh(&8, &fresh_ranges));
            assert!(is_fresh(&11, &fresh_ranges));
            assert!(is_fresh(&17, &fresh_ranges));
            assert!(!is_fresh(&32, &fresh_ranges));
        }
    }
}
