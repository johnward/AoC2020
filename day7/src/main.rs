use std::collections::HashMap;
use std::env::args;
use std::result::Result;

struct BagHolder {
    bags: HashMap<String, Bag>,
}

impl BagHolder {
    pub fn new() -> BagHolder {
        BagHolder {
            bags: HashMap<String, Bag>::new()
        }
    }
}

struct Bag {
    description: String,
    bags: Vec<Bag>,
}

impl Bag {
    pub fn new(description: String) -> Bag {
        Bag {
            description,
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

    const TEST_INPUT: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
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

    let content = String::from(&content);
    let part1_answer = part1(&content);

    let content = String::from(&content);
    let part2_answer = part2(&content);

    println!("Hello, world!");

    Ok(())
}
