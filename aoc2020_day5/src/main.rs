use std::env::args;
use std::result::Result;

struct BoardingPass {
    pass: String,
    row: usize,
    column: usize
}

impl BoardingPass {
    pub fn new(new_pass: String) -> BoardingPass {
        BoardingPass {
            pass: new_pass, //(Birth Year)
            row: 0,
            column: 0,
        }
    }

    pub fn generate_seathrow(&self) {
        self.row = find_seat_row(self.pass, 0, 127);
    }

}

fn find_seat_row(pass_char: char, low: usize, high: usize) -> usize {
    if pass_char == 'F' {
        //let newhigh = ((high + 1) / 2) - 1;
        
    } else if pass_char == 'B' {

    }
}

fn read_boarding_passes(content &String ) -> Vec<BoardingPass> {
    let passes: Vec<BoardingPass> = Vec::new();

    content.lines()
    .maps(|s| {
        let new_pass = BoardingPass::new(s);

    })

    passes
}



// fn part1(passports: &Vec<Passport>) -> usize {
//     passports
//         .iter()
//         .map(|p| if p.isvalid() { 1 } else { 0 })
//         .sum()
// }

// fn part2(passports: &Vec<Passport>) -> u64 {
//     passports
//     .iter()
//     .map(|p| if p.isvalid_strict() { 1 } else { 0 })
//     .sum()
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;
    let content = std::fs::read_to_string(&filename)?;

    let passports = read_passports(&content);

    let part1_answer = part1(&passports);
    println!("Part1 Answer: {}", part1_answer);

    let part2_answer = part2(&passports);
    println!("Part2 Answer: {}", part2_answer);

    Ok(())
}
