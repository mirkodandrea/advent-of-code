fn parse_ranges(s: &str) -> (u32, u32){
    let (low, high) = s.split_once('-').expect("should be a two elements tuple separated by a dash");
    let (low, high) = (low.parse::<u32>().expect("should be a number"), high.parse::<u32>().expect("should be a number"));
    (low, high)
}

fn main() {
    let overlapping = include_str!("../input.txt")
                    .split('\n')
                    .filter(|line| !line.is_empty())
                    .map(
                        |line| line.split_once(',')
                                    .expect("should be a two elements tuple separated by a comma")
                    ).map(
                        |(r1, r2)| (parse_ranges(r1), parse_ranges(r2))
                    )
                    .filter(
                        |((low1, high1), (low2, high2))| 
                            low2 <= high1 && low1 <= high2 || low1 <= high2 && low2 <= high1 
                    )
                    .count();
    println!("overlapping assignements: {}", overlapping);
}
