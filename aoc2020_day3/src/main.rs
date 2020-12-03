use std::env::args;
use std::result::Result;

fn part1(content: String) -> u32 {

    let mut string_position = 3;

    content.lines()
        .skip(1)
        .map(|x| {
            let mut count = 0;

            if x.as_bytes().iter().nth(string_position) == Some(&b'#'){
                count = 1;
            } 

            // increment the string or wrap it.
            if string_position + 3 > x.len()-1 {
                string_position = (string_position + 3) - x.len();
            }
            else {
                string_position += 3;
            }

            count
        }).sum()
}

fn part2(content: String) -> u32 {
    1
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testcase_part1() {
        //let input = [1721, 979, 366, 299, 675, 1456];
        //assert_eq!(part1(&input), 514579);
    }

    #[test]
    fn testcase_part2() {
        //let input = [1721, 979, 366, 299, 675, 1456];
        //assert_eq!(part2(&input), 241861950);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;

    let content = std::fs::read_to_string(&filename)?;

    let part1_answer = part1(content);

    println!("Part1 Answer: {}", part1_answer);

    Ok(())
}
