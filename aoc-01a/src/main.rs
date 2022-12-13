// import io library
use std::io::{self, BufRead};

fn main() {
    // open file input.txt in read-only mode
    let file = match std::fs::File::open("input.txt") {
        Err(why) => panic!("couldn't open input.txt: {}", why),
        Ok(file) => file,
    };
    // create a reader for reading the file line by line
    let mut reader = io::BufReader::new(file);

    // create a string to hold the line
    let mut line = String::new();

    let mut current_elf = 0;
    let mut current_elf_cals = 0;

    let mut max_elf_calories = 0;
    let mut max_elf = -1;
    
    //read the file line by line with a while loop
    while reader.read_line(&mut line).unwrap() > 0 {
        // trim the line
        let trimmed = line.trim().to_string();
        line.clear();

        // if the line is empty, go to next elf
        if trimmed.is_empty() {
            if current_elf_cals > max_elf_calories {
                max_elf_calories = current_elf_cals;
                max_elf = current_elf;
            }

            current_elf += 1;
            current_elf_cals = 0;
            continue;
        }

        // convert the line to an integer
        let number = match trimmed.parse::<i32>(){
            Err(why) => panic!("couldn't parse {}: {}", trimmed, why),
            Ok(number) => number,
        };
        // add the number to the total
        current_elf_cals += number;

    }
    if current_elf_cals > max_elf_calories {
        max_elf_calories = current_elf_cals;
        max_elf = current_elf;
    }

    println!("Elf {} has {} calories", max_elf, max_elf_calories);

}
