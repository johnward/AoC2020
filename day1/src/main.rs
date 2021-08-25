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
    let entries3: Vec<i32> = entries.clone();

    for number1 in entries.iter() {

        for number2 in entries2.iter() {
            for number3 in entries3.iter() {

                if (number1 + number2 + number3) == 2020 {

                    let answer = number1 * number2 * number3;
                    println!("Three Numbers are: {}, {}, {}", number1, number2, number3);
                    println!("Happy Days, answer is {}", answer);
                }
            }
        }
    }

    Ok(())
}
