use std::env::args;
use std::result::Result;

struct Bags {
    bags: HashMap<String, Bag>,
}

impl Bags {
    pub fn new() -> Bag {
        Bag {
            bags: HashMap<String, Bag>::new()
        }
    }
}

struct Bag {
    name: String,
    bags: Vev<Bag>,
}

impl Bag {
    pub fn new(name: String) -> Bag {
        Bag {
            name,
            bags: Vec<Bag>::new()
        }
    }
}

fn populate_bags(content: &String) {
    
}



fn part1(content: &String) -> usize {
    //customs_forms_anyone(content)
    0
}

fn part2(content: &String) -> usize {
    //customs_forms_everyone(content)
    0
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

fn main() {
    let filename = args().nth(1).ok_or("I need a filename")?;
    let content = std::fs::read_to_string(&filename)?;

    let content = String::from(TEST_INPUT);
    let part1_answer = part1(&content);

    let content = String::from(TEST_INPUT);
    let part2_answer = part2(&content);

    println!("Hello, world!");
}
