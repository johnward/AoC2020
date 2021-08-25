use itertools::Itertools;
use std::env::args;
use std::result::Result;

#[derive(PartialEq)]
enum Seat {
    ROW,
    COLUMN,
}

#[derive(Debug, Clone)]
struct BoardingPass {
    pass: String,
    row: usize,
    column: usize,
    seat_id: usize,
}

impl BoardingPass {
    pub fn new(new_pass: String) -> BoardingPass {
        BoardingPass {
            pass: new_pass, //(Birth Year)
            row: 0,
            column: 0,
            seat_id: 0,
        }
    }

    pub fn generate_seat_row(&mut self) -> usize {
        self.find_seat(Seat::ROW, 0, 127)
    }

    pub fn generate_seat_column(&mut self) -> usize {
        self.find_seat(Seat::COLUMN, 0, 7)
    }

    pub fn create_seat_id(&mut self) {
        self.seat_id = (self.row * 8) + self.column;
    }

    fn find_seat(&mut self, seat: Seat, input_low: usize, input_high: usize) -> usize {
        let mut low = input_low;
        let mut high = input_high;
        let mut row_number: usize = 0;
        let mut middle: f32 = 0.0;
        let mut found: bool = false;

        let input_string: &str = if seat == Seat::ROW {
            &self.pass[..7]
        } else {
            &self.pass[7..10]
        };

        input_string.chars().for_each(|s| {
            middle = (high as f32 - low as f32) / 2 as f32;

            match s {
                'B' | 'R' => {
                    // high
                    low = high - middle.floor() as usize;

                    if high == low && !found {
                        row_number = high;
                        found = true;
                    }
                }
                'F' | 'L' => {
                    // low
                    high = low + middle.floor() as usize;

                    if high == low && !found {
                        row_number = low;
                        found = true;
                    }
                }
                _ => println!("No match"),
            }
        });

        row_number
    }
}

fn read_boarding_passes_and_sort(content: &String) -> Result<Vec<BoardingPass>, &'static str> {
    let mut passes: Vec<BoardingPass> = Vec::new();

    let count: usize = content
        .lines()
        .map(|s| {
            let mut new_pass = BoardingPass::new(String::from(s));
            new_pass.row = new_pass.generate_seat_row();
            new_pass.column = new_pass.generate_seat_column();
            new_pass.create_seat_id();
            passes.push(new_pass);
            1
        })
        .sum();

    passes.sort_by(|a, b| a.seat_id.partial_cmp(&b.seat_id).unwrap());

    println!("Count: {}", count);

    Ok(passes)
}

fn part1(passes: &Vec<BoardingPass>) -> usize {
    passes.iter().map(|a| a.seat_id).max().unwrap()
}

fn part2(passes: &Vec<BoardingPass>) -> usize {
    let mut missing_id: usize = 0;

    for (pass1, pass2) in passes
        .iter()
        .filter(|a| (a.row > 0 && a.row < 127))
        .tuple_windows()
    {
        if (pass2.seat_id - pass1.seat_id) == 2 {
            missing_id = pass1.seat_id + 1;
        }
    }

    missing_id
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"FBFBBFFRLR"#;

    #[test]
    fn testcase1() {
        let content = String::from(TEST_INPUT);
        let passes = read_boarding_passes_and_sort(&content).unwrap();
        let count = part1(&passes);
        assert_eq!(count, 357);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;
    let content = std::fs::read_to_string(&filename)?;

    let boarding_passes = read_boarding_passes_and_sort(&content).unwrap();

    let part1_answer = part1(&boarding_passes);
    println!("Part1 Answer: {}", part1_answer);

    let part2_answer = part2(&boarding_passes);
    println!("Part2 Answer: {}", part2_answer);

    Ok(())
}
