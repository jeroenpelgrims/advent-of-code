use std::{collections::HashMap, fs};

type Update = Vec<i32>;
type Order = Vec<(i32, i32)>;

#[derive(Debug)]
struct Config {
    order: Order,
    updates: Vec<Update>,
}

fn parse_order_line(line: &str) -> (i32, i32) {
    let parts: Vec<_> = line
        .splitn(2, "|")
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    if let [a, b] = parts.as_slice() {
        (*a, *b)
    } else {
        panic!("Unsupported format for order line")
    }
}

fn parse_update_line(line: &str) -> Vec<i32> {
    line.split(",").map(|x| x.parse::<i32>().unwrap()).collect()
}

fn parse_input(input: &str) -> Config {
    let parts: Vec<_> = input.splitn(2, "\n\n").collect();
    if let [order, updates] = parts.as_slice() {
        let order = order.split("\n").map(parse_order_line).collect();
        let updates = updates.split("\n").map(parse_update_line).collect();
        Config { order, updates }
    } else {
        panic!("Unsupported format");
    }
}

fn is_valid(update: Update, order: &Order) -> bool {
    let relevant_order: Vec<_> = order
        .iter()
        .filter(|(a, b)| update.contains(a) && update.contains(b))
        .collect();
    update.is_sorted_by(|a, b| {
        let befores: Vec<_> = relevant_order
            .iter()
            .filter(|(x, y)| x == a)
            .map(|(x, y)| *y)
            .collect();
        befores.contains(b)
    })
}

fn sum_of_centers(xs: Vec<&Update>) -> i32 {
    let middles = xs.iter().map(|update| update[update.len() / 2]);
    middles.fold(0, |total, e| total + e)
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let config = parse_input(&input);
    let valids: Vec<_> = config
        .updates
        .iter()
        .filter(|update| is_valid(update.to_vec(), &config.order))
        .collect();
    let sum = sum_of_centers(valids);

    println!("1: {:?}", sum);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_is_safe() {}
// }
