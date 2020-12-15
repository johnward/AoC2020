use std::env::args;
use std::result::Result;
use std::collections::HashMap;

fn part1(content: &String) -> usize {
    customs_forms(content)
}

// fn part2(passports: &Vec<Passport>) -> u64 {
//     passports
//     .iter()
//     .map(|p| if p.isvalid_strict() { 1 } else { 0 })
//     .sum()
// }

fn count_questions(forms: &str) -> usize {
    let mut questions = HashMap::new();

    forms.chars().for_each(|s| {
                questions.insert(s, 1);
            });

    questions.values().sum()
}

fn customs_forms(content: &String) -> usize {
    content.split("\n\n").map(|s: &str| s.lines().map(count_questions).sum::<usize>()).sum::<usize>()
}

#[cfg(test)]
mod test {
    //use super::*;

    const TEST_INPUT: &str = r#"hgt:159cm
iyr:1997 byr:1994
hgt:189cm
ecl:gry pid:564165515
eyr:2029 hcl:#ceb3a1"#;

    #[test]
    fn testcase1() {
        // let content = String::from(TEST_INPUT);
        // let passports = read_passports(&content);
        // let count = part1(&passports);
        // assert_eq!(count, 2);
    }

    #[test]
    fn testcase2() {
        // let content = String::from(TEST_INPUT2);
        // let passports = read_passports(&content);
        // let count = part1(&passports);
        // assert_eq!(count, 5);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;
    let content = std::fs::read_to_string(&filename)?;

    let part1_answer = part1(&content);
    println!("Part1 Answer: {}", part1_answer);

    // let part2_answer = part2(&passports);
    // println!("Part2 Answer: {}", part2_answer);

    Ok(())
}