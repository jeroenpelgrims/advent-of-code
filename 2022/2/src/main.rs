use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Choice {
    Rock,
    Scissors,
    Paper,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum MatchResult {
    Win,
    Lose,
    Draw,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Game {
    opponent: Choice,
    player: Choice,
    result: MatchResult,
}

fn parse_opponent(c: &str) -> Choice {
    match c {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Unknown opponent choice"),
    }
}

fn parse_line(line: &str) -> Game {
    let (opponent_char, player_char) = line.split_once(' ').unwrap();
    let opponent = parse_opponent(opponent_char);
    let player = match player_char {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => panic!("Unknown player choice"),
    };
    let result = match (player, opponent) {
        (player, opponent) if player == opponent => MatchResult::Draw,
        (Choice::Rock, Choice::Scissors) => MatchResult::Win,
        (Choice::Rock, Choice::Paper) => MatchResult::Lose,
        (Choice::Paper, Choice::Rock) => MatchResult::Win,
        (Choice::Paper, Choice::Scissors) => MatchResult::Lose,
        (Choice::Scissors, Choice::Paper) => MatchResult::Win,
        (Choice::Scissors, Choice::Rock) => MatchResult::Lose,
        _ => panic!("Unknown match parameters"),
    };
    Game {
        opponent,
        player,
        result,
    }
}

fn parse_line2(line: &str) -> Game {
    let (opponent_char, result_char) = line.split_once(' ').unwrap();
    let opponent = parse_opponent(opponent_char);
    let result = match result_char {
        "X" => MatchResult::Lose,
        "Y" => MatchResult::Draw,
        "Z" => MatchResult::Win,
        _ => panic!("Unknown player choice"),
    };
    let player = match (result, opponent) {
        (result, opponent) if result == MatchResult::Draw => opponent,
        (MatchResult::Win, Choice::Scissors) => Choice::Rock,
        (MatchResult::Win, Choice::Paper) => Choice::Scissors,
        (MatchResult::Win, Choice::Rock) => Choice::Paper,
        (MatchResult::Lose, Choice::Scissors) => Choice::Paper,
        (MatchResult::Lose, Choice::Paper) => Choice::Rock,
        (MatchResult::Lose, Choice::Rock) => Choice::Scissors,
        _ => panic!("Unknown match parameters"),
    };
    Game {
        opponent,
        result,
        player,
    }
}

fn score_game(game: Game) -> i32 {
    let choice_score = match game.player {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    let result_score = match game.result {
        MatchResult::Lose => 0,
        MatchResult::Draw => 3,
        MatchResult::Win => 6,
    };
    choice_score + result_score
}

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    println!(
        "1: {:?}",
        text.lines().map(parse_line).map(score_game).sum::<i32>()
    );
    println!(
        "2: {:?}",
        text.lines().map(parse_line2).map(score_game).sum::<i32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let game1 = parse_line("A Y");
        assert_eq!(game1.opponent, Choice::Rock);
        assert_eq!(game1.player, Choice::Paper);
        assert_eq!(game1.result, MatchResult::Win);

        let game2 = parse_line("B X");
        assert_eq!(game2.opponent, Choice::Paper);
        assert_eq!(game2.player, Choice::Rock);
        assert_eq!(game2.result, MatchResult::Lose);

        let game3 = parse_line("C Z");
        assert_eq!(game3.opponent, Choice::Scissors);
        assert_eq!(game3.player, Choice::Scissors);
        assert_eq!(game3.result, MatchResult::Draw);
    }

    #[test]
    fn test_parse_line2() {
        let game1 = parse_line2("A Y");
        assert_eq!(game1.opponent, Choice::Rock);
        assert_eq!(game1.result, MatchResult::Draw);
        assert_eq!(game1.player, Choice::Rock);

        let game2 = parse_line2("B X");
        assert_eq!(game2.opponent, Choice::Paper);
        assert_eq!(game2.result, MatchResult::Lose);
        assert_eq!(game2.player, Choice::Rock);

        let game3 = parse_line2("C Z");
        assert_eq!(game3.opponent, Choice::Scissors);
        assert_eq!(game3.result, MatchResult::Win);
        assert_eq!(game3.player, Choice::Rock);
    }

    #[test]
    fn test_score_game() {
        assert_eq!(score_game(parse_line("A Y")), 8);
        assert_eq!(score_game(parse_line("B X")), 1);
        assert_eq!(score_game(parse_line("C Z")), 6);
    }
}
