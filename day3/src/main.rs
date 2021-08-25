use std::env::args;
use std::result::Result;

fn part1(content: &String, right: usize, down: usize) -> u64 {

    let mut string_position = right;

    content.lines()
        .skip(down)
        .step_by(down)
        .map(|x| {
            let mut count = 0;

            if x.as_bytes().iter().nth(string_position) == Some(&b'#'){
                count = 1;
            } 

            // increment the string or wrap it.
            if string_position + right > x.len()-1 {
                string_position = (string_position + right) - x.len();
            }
            else {
                string_position += right;
            }

            count
        }).sum()
}

fn part2(content: &String) -> u64 {
    let mut total:u64 = part1(&content, 1, 1);
    total *= part1(&content, 3, 1);
    total *= part1(&content, 5, 1);
    total *= part1(&content, 7, 1);
    total *= part1(&content, 1, 2);
    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;

    let content = std::fs::read_to_string(&filename)?;

    let part1_answer = part1(&content, 3, 1);

    println!("Part1 Answer: {}", part1_answer);

    let part2_answer = part2(&content);

    println!("Part1 Answer: {}", part2_answer);

    Ok(())
}
