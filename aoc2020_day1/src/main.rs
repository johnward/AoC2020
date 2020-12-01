use std::env::args;
use std::result::Result;

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;

    let content = std::fs::read_to_string(&filename)?;

    let mut entries = Vec::new();

    // Read file into Vec
    for line in content.lines() {
        let number: i32 = line.parse().unwrap_or(0);
        entries.push(number);
    }

    let entries2: Vec<i32> = entries.clone();

    for base_number in entries.iter() {

        for comp_number in entries2.iter() {
            if (base_number + comp_number) == 2020 {

                let answer = base_number * comp_number;
                println!("Happy Days, answer is {}", answer);
            }
        }
    }

    Ok(())
}
