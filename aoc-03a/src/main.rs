use std::collections::HashSet;

pub trait Priority {
    fn priority(&self) -> i64;
}

impl Priority for char {
    fn priority(&self) -> i64 {
        match self {
            'a'..='z' => return(*self as u8 - 'a' as u8 + 1).into(),
            'A'..='Z' => return (*self as u8 - 'A' as u8 + 27).into(),
            _ => panic!("Invalid input"),
        }        
    }
}

fn find_duplicates(str1: &str, str2: &str) -> HashSet<char> {
    let mut result = HashSet::new();
    for c1 in str1.chars(){
        if str2.contains(c1) {
            result.insert(c1);
        }
    }
    //println!("duplicates: {:?}", result);
    result
}

fn split_in_half(the_str: &str) -> (&str, &str) {
    let ln = the_str.len()/2;
    let (left, right) = the_str.split_at(ln);
    //println!("left: {}, right: {}", left, right);
    
    (left, right)
}



fn main() {
    let rucksacks = include_str!("../input.txt").split('\n').collect::<Vec<&str>>();
    
    let value = rucksacks.iter()
        .map(|rucksack| split_in_half(rucksack))
        .map(|(left, right)| find_duplicates(left, right))        
        .map(|duplicates| duplicates.iter().map(|c| c.priority()).sum::<i64>())
        .sum::<i64>();
    
    println!("sum of priorities: {}", value);
}
    
