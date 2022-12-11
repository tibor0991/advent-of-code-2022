/// Struct representing a single move from any character
#[derive(Clone, Copy)]
enum Move {
    Rock = 1, 
    Paper = 2, 
    Scissors = 3
}

// Converts from a string to a known Move value
impl TryFrom<&str> for Move {
    type Error = String;
    fn try_from(orig: &str) -> Result<Self, Self::Error> {
        match orig {
            "A"|"X" => Ok(Move::Rock),
            "B"|"Y" => Ok(Move::Paper),
            "C"|"Z" => Ok(Move::Scissors),
            _ => Err(format!("Invalid conversion: {orig} is not mapped to a move!"))
        }
    }
}

// Outcome of a throw
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3
}

// There is a smarter way to go about this, however I'm toying around.
// Besides, the cases are few and well defined, might as well enumerate all possible outcomes.
impl From<(Move, Move)> for Outcome {
    fn from(pair: (Move, Move)) -> Self {
        // Slight mistake: moves in the 'strategy guide' come opponent-first
        // but the pattern matching assumes that the lhs item is me
        let (oppo_move, my_move) = pair;
        match (my_move, oppo_move) {
            (Move::Paper, Move::Scissors) => Outcome::Loss,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Loss,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (_, _) => Outcome::Draw
        }
    }
}

// Safe and backpropagating function to convert a pair of strings into a pair of moves.
// If it fails I can bring the error to stdout.
fn convert_line2tuple(line: (&str, &str)) -> Result<(Move, Move), String>{
    let (left, right) = line;
    match (Move::try_from(left), Move::try_from(right)) {
        (Ok(left_move), Ok(right_move)) => Ok((left_move, right_move)),
        (_, Err(right_move)) => Err(right_move),
        (Err(left_move), _) => Err(left_move)
    }
}

use std::env;
use std::fs;

fn main() -> Result<(), String>{
    // Colect args from command line
    let args = env::args().collect::<Vec<_>>();

    // Path to the strategy guide (or return an error)
    let guide_path = args.get(1).ok_or("No path provided!")?;

    // Opens the strategy guide from a text file
    let strategy_guide = fs::read_to_string(&guide_path)
        .map_err(|err| format!("Cannot open file {guide_path}: {}", err.to_string()))?;

    //let strategy_guide = String::from("A Y\nB X\nC Z");

    let final_result: u8 = strategy_guide
        // Splits the input along newlines
        .split('\n')
        // Transforms each line
        .map(|line| line
            // Convert line to an option of 2-tuple
            .split_once(' ')
            // Convert the option to a result
            .ok_or(format!("Unable to read line {line}"))
            // Convert the (&str, &str) 2-tuple to a (Move, Move) 2-tuple
            .map(|split_line| convert_line2tuple(split_line))?
            // Convert each (Move, Move) tuple to a (Outcome, Move) tuple
            // Outcome is relative to the main character
            // Move is the move played by the main character
            .map(|pair| (Outcome::from(pair), pair.1))
            // Sum the value of the outcome to the value of the played move to obtain the score
            .map(|(outcome, my_move):(Outcome, Move)| (my_move as u8) + (outcome as u8) ))
        // Collect all scores as Result, in order to back-propagate errors to the caller
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        // Sum all scores
        .sum();

    println!("The outcome of this run is {final_result}!");

    Ok(())
}
