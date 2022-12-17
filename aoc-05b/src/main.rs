
#[derive(Debug)]
struct State {
    stacks: Vec<Vec<char>>
}

impl State {
    fn new(size: usize) -> Self {
        let mut obj = Self {
            stacks: Vec::new()
        };
        for _ in 0..size {
            obj.stacks.push(Vec::new());
        }
        obj
    }

    fn move_crates(&mut self, from: usize, to: usize, count: usize) {
        let crates = (0..count).map(
            |_| self.stacks[from-1].pop().unwrap()
        ).collect::<Vec<char>>();
        for cr in crates.iter().rev() {
            self.stacks[to-1].push(*cr);
        }
    }

    fn top(&self) -> String{
        let top_str = self.stacks.iter().map(|stack| stack.last().unwrap_or(&' ')).collect();
        top_str
    }

}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        let mut lines = s.split('\n').collect::<Vec<&str>>();        
        let count = lines.pop().unwrap().split_whitespace().count();

        let mut obj = Self::new(count);

        for line in lines.iter().rev(){
            for i in 0..count {
                let  index = (i*4)+1;
                let c = line.chars().nth(index).unwrap();
        
                if c != ' ' {
                    obj.stacks[i].push(c);
                    
                }               
            }
        }
        
        obj
    }
}


fn main() {
    let input = include_str!("../input.txt");

    let (state_text, moves) = input.split_once("\n\n").unwrap();
    
    let mut state = State::from(state_text);
    
    moves.split('\n')
    .filter(|line| !line.is_empty())
    .map(
        |move_line| {
            let parts = move_line.splitn(6, " ").collect::<Vec<&str>>();
            (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5])
        }
    ).map(
        |(_, num, _, from, _, to)| (
            num.parse::<usize>().expect("should be a number"), 
            from.parse::<usize>().expect("should be a number"),
            to.parse::<usize>().expect("should be a number"),
        )
    ).for_each(
        |(num, from, to)| {
            state.move_crates(from, to, num);
        }
    );

    println!("top: {}", state.top());
    
}
