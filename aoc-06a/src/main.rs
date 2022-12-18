const N_CHARS: usize = 4;

fn main() {
    let input = include_str!("../input.txt");
    (N_CHARS..=input.len())
        .take_while(|i| {
            let i = *i;
            let mut chars: Vec<char> = input[i - N_CHARS..i].chars().collect::<Vec<char>>();

            chars.sort();
            chars.dedup();

            chars.len() < N_CHARS
        })
        .last()
        .map(|i| println!("{}", i + 1));
}
