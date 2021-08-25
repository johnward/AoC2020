use std::env::args;
use std::result::Result;
use regex::Regex;

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
        self.byr != ""
            && self.iyr != ""
            && self.eyr != ""
            && self.hgt != ""
            && self.hcl != ""
            && self.ecl != ""
            && self.pid != ""
    }

    pub fn isvalid_strict(&self) -> bool {
        let mut isvalid = false;
        
        let re = Regex::new(r"^\d{4}$").unwrap();
        if re.is_match(self.byr.as_str()) {
            let number: i32 = self.byr.as_str().parse().unwrap_or(0);

            isvalid = number >= 1920 && number <= 2002 
        } else {
            isvalid = false;
        }

        if isvalid && re.is_match(self.iyr.as_str()) {
            let number: i32 = self.iyr.as_str().parse().unwrap_or(0);

            isvalid = number >= 2010 && number <= 2020;
        } else {
            isvalid = false;
        }

        if isvalid && re.is_match(self.eyr.as_str()) {
            let number: i32 = self.eyr.as_str().parse().unwrap_or(0);

            isvalid = number >= 2020 && number <= 2030;
        } else {
            isvalid = false;
        }

        let re_cm = Regex::new(r"^\d{3}cm$").unwrap();
        let re_in = Regex::new(r"^\d{2}in$").unwrap();

        if isvalid && re_cm.is_match(self.hgt.as_str()) {
            let cm_height = &self.hgt[0..3];
            let number: i32 = cm_height.parse().unwrap_or(0);

            isvalid = number >= 150 && number <= 193;
        } else if isvalid && re_in.is_match(self.hgt.as_str()) {
            let in_height = &self.hgt[0..2];
            let number: i32 = in_height.parse().unwrap_or(0);

            isvalid = number >= 59 && number <= 76;
        } else {
            isvalid = false;
        }

        if isvalid {
            let re = Regex::new(r"^#[a-fA-F0-9]{6,}$").unwrap();
            isvalid = re.is_match(self.hcl.as_str());
        } else {
            isvalid = false;
        }

        //amb blu brn gry grn hzl oth
        if isvalid {
            isvalid = self.ecl == "amb" || self.ecl == "blu" || self.ecl == "brn"  
            || self.ecl == "gry" || self.ecl == "grn"|| self.ecl == "hzl" || self.ecl == "oth";
        } else {
            isvalid = false;
        }

        if isvalid {
            let re = Regex::new(r"^\d{9}$").unwrap();
            isvalid = re.is_match(self.pid.as_str());
        } else {
            isvalid = false;
        }

        isvalid
    }
}

fn part1(passports: &Vec<Passport>) -> usize {
    passports
        .iter()
        .map(|p| if p.isvalid() { 1 } else { 0 })
        .sum()
}

fn part2(passports: &Vec<Passport>) -> u64 {
    passports
    .iter()
    .map(|p| if p.isvalid_strict() { 1 } else { 0 })
    .sum()
}

fn read_passports(content: &String) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport_count = 0;
    let mut passport = Passport::new();
    let mut valid_passport = 0;

    let count: usize = content.split(|c| c == ' ' || c == '\n')
        .map(|s| {
            if s == "" {
                passport_count += 1;
                passports.push(passport.clone()); // push existing passport as it is complete
                passport = Passport::new(); // create a new passport
                valid_passport = 1;
            } else {
                let key_values: Vec<&str> = s.split(|t| t == ':').collect();
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
                        _ => println!("Error"),
                    }
                }
                valid_passport = 0;
            }

            valid_passport
        })
        .sum();

    println!("Count: {}", count);

    passports
}

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
        let passports = read_passports(&content);
        let count = part1(&passports);
        assert_eq!(count, 2);
    }

    #[test]
    fn testcase2() {
        let content = String::from(TEST_INPUT2);
        let passports = read_passports(&content);
        let count = part1(&passports);
        assert_eq!(count, 5);
    }

    #[test]
    fn testcase3() {
        let content = String::from(TEST_INPUT);
        let passports = read_passports(&content);
        let count = part2(&passports);
        assert_eq!(count, 5);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;
    let content = std::fs::read_to_string(&filename)?;

    let passports = read_passports(&content);

    let part1_answer = part1(&passports);
    println!("Part1 Answer: {}", part1_answer);

    let part2_answer = part2(&passports);
    println!("Part2 Answer: {}", part2_answer);

    Ok(())
}
