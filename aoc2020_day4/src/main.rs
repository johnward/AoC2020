use std::env::args;
use std::result::Result;


#[derive(Debug, Clone)]
struct Passport {
    byr: String, //(Birth Year)
    iyr: String, //(Issue Year)
    eyr: String, //(Expiration Year)
    hgt: String, //(Height)
    hcl: String, //(Hair Color)
    ecl: String, //(Eye Color)
    pid: String, //(Passport ID)
    cid: String, //(Country ID) - Optional
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            byr: String::from(""), //(Birth Year)
            iyr: String::from(""), //(Issue Year)
            eyr: String::from(""), //(Expiration Year)
            hgt: String::from(""), //(Height)
            hcl: String::from(""), //(Hair Color)
            ecl: String::from(""), //(Eye Color)
            pid: String::from(""), //(Passport ID)
            cid: String::from(""),    
        }
    }

    pub fn isvalid(&self) -> bool {
        self.byr != "" && self.iyr != "" && self.eyr != "" && self.hgt != "" && self.hcl != "" && self.ecl != "" && self.pid != ""
    }
}

fn part1(passports: Vec<Passport>) -> usize {
    passports.iter()
        .map(|p| {
            if p.isvalid() {
                1
            } else {
                0
            }
        }).sum()
}

// fn part1(content: &String) -> (u64, Vec<HashMap<String, String>>) {

//     let mut valid_passports = 0;
//     let mut passports_processed = 0;

//     let passports = content.split_whitespace()
//         .map(|s| {
//                 if s != "" {
//                     let key_values: Vec<&str>= s.split(|t| t == ':').collect();
//                     if key_values.len() == 2 {
//                     match key_values[0] {
//                         "byr" => byr = true,
//                         "iyr" => iyr = true,
//                         "eyr" => eyr = true,
//                         "hgt" => hgt = true,
//                         "hcl" => hcl = true,
//                         "ecl" => ecl = true,
//                         "pid" => pid = true,
//                         "cid" => cid = true,
//                         _      => {
//                                 println!("Error")
//                                 }
//                         }
//                     } 
//                     if byr && iyr && eyr && hgt && hcl && ecl && pid
//                     {
//                         valid_passports += 1;
//                     }
//                 }
//                 else
//                 {
//                     passports_processed += 1;
//                     byr = false; //(Birth Year)
//                     iyr = false; //(Issue Year)
//                     eyr = false; //(Expiration Year)
//                     hgt = false; //(Height)
//                     hcl = false; //(Hair Color)
//                     ecl = false; //(Eye Color)
//                     pid = false; //(Passport ID)
//                     cid = false; //(Country ID) - Optional
//                 }
                
//                 (String::from(key_values[0]), String::from(key_values[1]))
//         }).collect();

//         (valid_passports, tokens)

// }

fn read_passports(content: &String) -> Vec<Passport> {

    let mut passports: Vec<Passport> = Vec::new();
    let mut passport_count = 0;
    let mut passport = Passport::new();

    let count: usize = content.split_whitespace()
    .map( |s| {
        if s == "" {
            passport_count += 1;
            passports.push(passport.clone()); // push existing passport as it is complete
            passport = Passport::new(); // create a new passport
        } else {
            let key_values: Vec<&str>= s.split(|t| t == ':').collect();
                    if key_values.len() == 2 {
                    match key_values[0] {
                        "byr" => passport.byr = String::from(key_values[1]),
                        "iyr" => passport.iyr = String::from(key_values[1]),
                        "eyr" => passport.eyr = String::from(key_values[1]),
                        "hgt" => passport.hgt = String::from(key_values[1]),
                        "hcl" => passport.hcl = String::from(key_values[1]),
                        "ecl" => passport.ecl = String::from(key_values[1]),
                        "pid" => passport.pid = String::from(key_values[1]),
                        "cid" => passport.cid = String::from(key_values[1]),
                        _ => println!("Error")
                        
                    }
                }
            }
            
            passport_count
        }).sum();

    println!("Count: {}", count);
    
    passports
}

// fn part2(content: &String) -> u64 {
//     0
// }


#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

    const TEST_INPUT2: &str = r#"hgt:159cm
iyr:1997 byr:1994
hgt:189cm
ecl:gry pid:564165515
eyr:2029 hcl:#ceb3a1

hcl:#602927 eyr:2024
cid:130
iyr:2015 ecl:blu
hgt:184cm byr:1996
pid:897871188

pid:561068005 eyr:2025 iyr:2017 cid:139 ecl:blu hcl:#ceb3a1
byr:1940

iyr:2014
byr:1986 pid:960679613 eyr:2025 ecl:hzl

cid:211 ecl:blu hcl:#7d3b0c iyr:2011 pid:006632702
byr:1982 eyr:2023 hgt:68in

hcl:#341e13 hgt:192 iyr:2028
ecl:utc
eyr:2027 byr:1979 pid:653515689

eyr:2026 hgt:161cm ecl:#1850b8
pid:298779494 hcl:b2114e iyr:1953

hgt:155cm
hcl:#a97842 iyr:2019
ecl:gry byr:1939
pid:935099157 eyr:2027
"#;

    #[test]
    fn testcase1() {
        let content = String::from(TEST_INPUT);
        let count = part1(&content);
        assert_eq!(count, 2);
    }

    #[test]
    fn testcase2() {
        let content = String::from(TEST_INPUT2);
        let count = part1(&content);
        assert_eq!(count, 5);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;

    let content = std::fs::read_to_string(&filename)?;

    let passports = read_passports(&content);

    //let mut part1_answer: u64 = 0;
    //let mut passports: Vec<HashMap<String, String>> = Vec::new();

    let part1_answer = part1(passports);

    println!("Part1 Answer: {}", part1_answer);

    // let part2_answer = part2(&content);

    // println!("Part1 Answer: {}", part2_answer);

    Ok(())
}
