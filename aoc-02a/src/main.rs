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
    // A, X is rock
    // B, Y is paper    
    // C, Z is scissors

    match game {
        ("A", "X") => DRAW + ROCK, // rock vs rock -> draw
        ("B", "X") => LOSE + ROCK, // paper vs rock -> lose
        ("C", "X") => WIN + ROCK, // scissors vs rock -> win
        ("A", "Y") => WIN + PAPER, // rock vs paper -> win
        ("B", "Y") => DRAW + PAPER, // paper vs paper -> draw
        ("C", "Y") => LOSE + PAPER, // scissors vs paper -> lose
        ("A", "Z") => LOSE + SCISSORS, // rock vs scissors -> lose
        ("B", "Z") => WIN + SCISSORS, // paper vs scissors -> win
        ("C", "Z") => DRAW + SCISSORS, // scissors vs scissors -> draw
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
                //.collect::<Vec<(&str, &str)>>();
    println!("Score: {}", score);
}
