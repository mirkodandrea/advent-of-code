use std::fs;

const WIN: i64 = 6;
const DRAW: i64 = 3;
const LOSE: i64 = 0;
const ROCK: i64 = 1;
const PAPER: i64 = 2;
const SCISSORS: i64 = 3;


fn get_columns(line: &str) -> Vec<&str>{
    line.split(" ").collect()
}

fn get_score(game: (&str, &str)) -> i64 {
    // A is rock
    // B is paper    
    // C is scissors
    // X means lose
    // Y means draw
    // Z means win

    match game {
        ("A", "X") => LOSE + SCISSORS, // Lose by playing scissors vs rock
        ("B", "X") => LOSE + ROCK, // Lose by playing rock vs paper
        ("C", "X") => LOSE + PAPER, // Lose by playing paper vs scissors
        ("A", "Y") => DRAW + ROCK, // Draw by playing rock vs rock
        ("B", "Y") => DRAW + PAPER, // Draw by playing paper vs paper
        ("C", "Y") => DRAW + SCISSORS, // Draw by playing scissors vs scissors
        ("A", "Z") => WIN + PAPER, // Win by playing Paper vs rock
        ("B", "Z") => WIN + SCISSORS, // Win by playing Scissors vs Paper
        ("C", "Z") => WIN + ROCK, // Win by playing Rock vs Scissors
        _ => panic!("Invalid input"),
    }
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let score:i64 = contents.split('\n')
                .map(|line| get_columns(line))
                .filter(|vals| vals.len() == 2)
                .map(|vals| (vals[0], vals[1]))
                .map(|game| get_score(game))
                .sum();

    println!("Score: {}", score);
}
