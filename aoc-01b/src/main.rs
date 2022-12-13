use std::fs;


fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    

    // split lines by newline
    let lines: Vec<&str> = contents.split('\n').collect();
    println!("Lines: {:?}", lines);
    
    let mut max_values = [0, 0, 0];
    let mut current = 0;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            for i in 0..3 {
                if current > max_values[i] {
                    max_values[i] = current;
                    break;
                }
            }       
            current = 0;
            continue;
        }

        let value = match trimmed.parse::<i64>(){
            Err(why) => panic!("couldn't parse {}: {}", trimmed, why),
            Ok(number) => number,
        };
        current += value;
        
    }

    print!("sum of max values: {}", max_values.iter().sum::<i64>());
}
