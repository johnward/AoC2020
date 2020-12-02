use std::env::args;
use std::result::Result;

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;

    let content = std::fs::read_to_string(&filename)?;

    for line in content.lines() {
        let comps: Vec<&str> = line.split(':').collect();
        let rules = comps[0];
        let password = comps[1];

        println!("rules: {}", rules);
        println!("password: {}", password);

        let rule_comps: Vec<&str> = rules.split(' ').collect();

        let letter = rule_comps[1];
        let nums: Vec<&str> = rule_comps[0].split("-").collect();

        println!("Letter: {}", letter);
        println!("Nums: {} {}", nums[0], nums[1]);
    }

    Ok(())
}
