use itertools::Itertools;
use std::{cmp::Ordering, fs};

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
    update.is_sorted_by(|a, b| get_order(a, b, order) == Ordering::Less)
}

fn sum_of_centers(xs: Vec<Update>) -> i32 {
    let middles = xs.iter().map(|update| update[update.len() / 2]);
    middles.fold(0, |total, e| total + e)
}

fn get_order(a: &i32, b: &i32, order: &Order) -> Ordering {
    let order_pair = order
        .iter()
        .filter(|(x, y)| x == a && y == b || x == b && y == a)
        .next();
    match order_pair {
        Some((x, y)) if x == a && y == b => Ordering::Less,
        Some((x, y)) if x == b && y == a => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let config = parse_input(&input);
    let (valids, invalids) = config
        .updates
        .into_iter()
        .partition(|update| is_valid(update.to_vec(), &config.order));

    println!("1: {:?}", sum_of_centers(valids));

    let fixed_invalids: Vec<Update> = invalids
        .iter()
        .map(|update| {
            update
                .iter()
                .sorted_by(|a, b| get_order(a, b, &config.order))
                .map(|x| *x)
                .collect::<Update>()
        })
        .collect();
    println!("1: {:?}", sum_of_centers(fixed_invalids));
}
