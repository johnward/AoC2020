use std::collections::HashMap;
use std::env::args;
use std::result::Result;

fn part1(content: &String) -> usize {
    customs_forms_anyone(content)
}

fn part2(content: &String) -> usize {
    customs_forms_everyone(content)
}

fn customs_forms_anyone(content: &String) -> usize {
    let mut questions: HashMap<char, usize> = HashMap::new();
    let mut count: usize = 0;

    content.lines().for_each(|s: &str| {
        if s == "" {
            count += questions.values().sum::<usize>();
            questions = HashMap::new();
        } else {
            s.chars().filter(|x| x.is_ascii_alphabetic()).for_each(|t| {
                questions.insert(t, 1);
                ()
            });
        }
    });

    count
}

fn customs_forms_everyone(content: &String) -> usize {
    let mut questions: HashMap<char, usize> = HashMap::new();
    let mut count: usize = 0;
    let mut people: usize = 0;

    content.lines().for_each(|s: &str| {
        if s == "" {
            count += questions
                .values()
                .map(|v| if *v == people { 1 } else { 0 })
                .sum::<usize>();
            questions = HashMap::new();
            people = 0;
        } else {
            s.chars().filter(|x| x.is_ascii_alphabetic()).for_each(|t| {
                match questions.get(&t) {
                    Some(v) => questions.insert(t, v + 1),
                    None => questions.insert(t, 1),
                };

                ()
            });
            people += 1;
        }
    });

    count
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b

"#;

    #[test]
    fn testcase1() {
        let content = String::from(TEST_INPUT);
        let part1_answer = part1(&content);
        assert_eq!(part1_answer, 11);
    }

    #[test]
    fn testcase2() {
        let content = String::from(TEST_INPUT);
        let part2_answer = part2(&content);
        assert_eq!(part2_answer, 6);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;
    let content = std::fs::read_to_string(&filename)?;

    let part1_answer = part1(&content);
    println!("Part1 Answer: {}", part1_answer);

    let part2_answer = part2(&content);
    println!("Part2 Answer: {}", part2_answer);

    Ok(())
}
