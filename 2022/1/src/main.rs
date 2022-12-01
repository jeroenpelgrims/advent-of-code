use itertools::Itertools;
use std::vec::IntoIter;
use std::{fs, iter::Rev};

fn group_elves(path: &str) -> Vec<Vec<i32>> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .fold(vec![vec![]], |mut result: Vec<Vec<i32>>, item| {
            if item.is_empty() {
                result.push(vec![]);
            } else {
                let int_value = item.parse::<i32>().unwrap();
                result.last_mut().unwrap().push(int_value);
            }
            result
        })
}

fn sort_elves(elves: Vec<Vec<i32>>) -> Rev<IntoIter<i32>> {
    elves
        .iter()
        .map(|elve| elve.iter().sum::<i32>())
        .sorted()
        .rev()
}

fn main() {
    let top3 = sort_elves(group_elves("./input.txt")).take(3);
    let first = top3.clone().next().unwrap();
    println!("1: {:?}", first);
    println!("2: {:?}", top3.sum::<i32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_elves() {
        let elves = group_elves("./test_input.txt");
        assert_eq!(elves[0], vec![1000, 2000, 3000] as Vec<i32>);
        assert_eq!(elves[1], vec![4000] as Vec<i32>);
        assert_eq!(elves[2], vec![5000, 6000] as Vec<i32>);
        assert_eq!(elves[3], vec![7000, 8000, 9000] as Vec<i32>);
        assert_eq!(elves[4], vec![10000] as Vec<i32>);
    }

    #[test]
    fn test_sort_elves() {
        let elves = group_elves("./test_input.txt");
        let sorted = sort_elves(elves).collect::<Vec<i32>>();
        assert_eq!(sorted, vec![24000, 11000, 10000, 6000, 4000] as Vec<i32>);
    }

    // #[test]
    // fn test_parse_action() {}
}
