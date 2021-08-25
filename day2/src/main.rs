use std::env::args;
use std::result::Result;

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;

    let content = std::fs::read_to_string(&filename)?;

    let mut password_count = 0;

    for line in content.lines() {

        // split into password and rulesS
        let comps: Vec<&str> = line.split(':').collect();
        let rules = comps[0];
        let password: &str = comps[1].trim();

        println!("rules: {}", rules);
        println!("password: {}", password);

        // split up the high and low chars positions, and the actual char
        let rule_comps: Vec<&str> = rules.split(' ').collect();

        let letter = rule_comps[1];
        let nums: Vec<&str> = rule_comps[0].split("-").collect();
        let number_low: usize = nums[0].parse().unwrap_or(0);
        let number_high: usize = nums[1].parse().unwrap_or(0);

        println!("Letter: {}", letter);
        println!("Nums: {} {}", nums[0], nums[1]);

        // Get the chars at the two positions
        let pass_string = String::from(password);
        let str_one = pass_string.chars().nth(number_low-1).unwrap();
        let str_two = pass_string.chars().nth(number_high-1).unwrap();

        // Check if the char is in one of these positions
        if str_one.eq(&letter.chars().nth(0).unwrap()) ^ str_two.eq(&letter.chars().nth(0).unwrap()) {
            password_count += 1;
        } 
    }

    println!("Valid Passwords: {}", password_count);

    Ok(())
}
