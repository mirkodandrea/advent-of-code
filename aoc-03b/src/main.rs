use std::collections::HashMap;
use itertools::Itertools;

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


fn find_badge(rucksacks: &[&str]) -> char {
    let mut set: HashMap<char, u8> = HashMap::new();

    rucksacks.iter()
            .map(|r| r.chars().unique())
            .flat_map(|c| c)
            .for_each(
                |c|  {
                    if set.contains_key(&c) {
                        set.insert(c, set.get(&c).unwrap() + 1);
                    } else {
                        set.insert(c, 1);
                    }
                }
            );
            
    
    set.iter().filter(|(_k, v)| **v == 3).map(|(k, _v)| *k).next().unwrap_or(' ')
}


fn main() {
    let rucksacks = include_str!("../input.txt").split('\n').collect::<Vec<&str>>();
    
    let value = (rucksacks.chunks(3))
        .into_iter()
        .map(|rucksack| find_badge(rucksack))
        .filter(|c| *c != ' ')
        .map(|c| c.priority())
        .sum::<i64>();
    
    println!("values {}", value);
    
}
    
