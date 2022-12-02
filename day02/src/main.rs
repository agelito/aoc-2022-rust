use std::fs;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Draw,
    Win,
    Loss,
}

enum MoveOrOutcome {
    Move(Move),
    Outcome(Outcome)
}

fn move_for_outcome(opponent_move: Move, desired_outcome: Outcome) -> Move {
    match (opponent_move, desired_outcome) {
        (_, Outcome::Draw) => opponent_move,
        (Move::Paper, Outcome::Win) => Move::Scissor,
        (Move::Paper, Outcome::Loss) => Move::Rock,
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Loss) => Move::Scissor,
        (Move::Scissor, Outcome::Win) => Move::Rock,
        (Move::Scissor, Outcome::Loss) => Move::Paper,
    }
}

fn decode_move(code: char) -> Option<Move> {
    match code {
        'A' => Some(Move::Rock),
        'B' => Some(Move::Paper),
        'C' => Some(Move::Scissor),
        'X' => Some(Move::Rock),
        'Y' => Some(Move::Paper),
        'Z' => Some(Move::Scissor),
        _ => None
    }
}

fn decode_move_or_outcome(code: char) -> Option<MoveOrOutcome> {
    match code {
        'A' => Some(MoveOrOutcome::Move(Move::Rock)),
        'B' => Some(MoveOrOutcome::Move(Move::Paper)),
        'C' => Some(MoveOrOutcome::Move(Move::Scissor)),
        'X' => Some(MoveOrOutcome::Outcome(Outcome::Loss)),
        'Y' => Some(MoveOrOutcome::Outcome(Outcome::Draw)),
        'Z' => Some(MoveOrOutcome::Outcome(Outcome::Win)),
        _ => None,
    }
}

fn determine_outcome(moves: (Move, Move)) -> Outcome {
    match moves {
        (Move::Rock, Move::Rock) => Outcome::Draw,
        (Move::Paper, Move::Paper) => Outcome::Draw,
        (Move::Scissor, Move::Scissor) => Outcome::Draw,
        (Move::Rock, Move::Paper) => Outcome::Loss,
        (Move::Paper, Move::Scissor) => Outcome::Loss,
        (Move::Scissor, Move::Rock) => Outcome::Loss,
        (Move::Rock, Move::Scissor) => Outcome::Win,
        (Move::Paper, Move::Rock) => Outcome::Win,
        (Move::Scissor, Move::Paper) => Outcome::Win,
    }
}

fn determine_move_score(my_move: Move) -> u32 {
    match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3,
    }
}

fn determine_outcome_score(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn part_1(lines: &Vec<&str>) {
    let mut total_score = 0;

    for line in lines {
        let mut decoded_moves = Vec::new();

        let characters = line.chars();
        for code in characters {
            if let Some(decoded_move) = decode_move(code) {
                decoded_moves.push(decoded_move)
            }
        }

        assert_eq!(decoded_moves.len(), 2);

        let moves = (decoded_moves[1], decoded_moves[0]);
        let outcome = determine_outcome(moves);

        let score = determine_move_score(moves.0) +
            determine_outcome_score(outcome);

        total_score += score;
    }

    println!("part 1: {}", total_score);
}

fn part_2(lines: &Vec<&str>) {
    let mut total_score = 0;

    for line in lines {
        let mut opponent_move: Option<Move> = None;
        let mut desired_outcome: Option<Outcome> = None;

        let characters = line.chars();
        for code in characters {
            match decode_move_or_outcome(code) {
                Some(MoveOrOutcome::Move(m)) => opponent_move = Some(m),
                Some(MoveOrOutcome::Outcome(o)) => desired_outcome = Some(o),
                _ => { }
            }
        }

        let my_move = move_for_outcome(
            opponent_move.unwrap(), 
            desired_outcome.unwrap());

        let score = determine_move_score(my_move) +
            determine_outcome_score(desired_outcome.unwrap());

        total_score += score;
    }

    println!("part 2: {}", total_score);
}

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("Couldn't find input file.");
    let lines = file_content
        .split("\n")
        .collect::<Vec<_>>();

    part_1(&lines);
    part_2(&lines);
}