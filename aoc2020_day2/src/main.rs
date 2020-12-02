use std::env::args;
use std::result::Result;

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;

    let content = std::fs::read_to_string(&filename)?;

    let mut password_count = 0;

    for line in content.lines() {
        let comps: Vec<&str> = line.split(':').collect();
        let rules = comps[0];
        let password: &str = comps[1];

        println!("rules: {}", rules);
        println!("password: {}", password);

        let rule_comps: Vec<&str> = rules.split(' ').collect();

        let letter = rule_comps[1];
        let nums: Vec<&str> = rule_comps[0].split("-").collect();
        let number_low: usize = nums[0].parse().unwrap_or(0);
        let number_high: usize = nums[1].parse().unwrap_or(0);

        println!("Letter: {}", letter);
        println!("Nums: {} {}", nums[0], nums[1]);

        let matches: Vec<&str> = password.matches(letter).collect();
        if number_low <= matches.iter().count() && matches.iter().count() <= number_high {
            password_count += 1;
        } 
    }

    println!("Valid Passwords: {}", password_count);

    Ok(())
}
