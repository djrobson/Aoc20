use std::collections::HashMap;

fn check_byr(s: &str) -> bool {
    match s.parse::<u32>() {
        Ok(yr) if yr >=1920 && yr <=2002  => true,
        _ => false,
    }
}
fn check_iyr(s: &str) -> bool {
    match s.parse::<u32>() {
        Ok(yr) if yr >=2010 && yr <=2020  => true,
        _ => false,
    }
}
fn check_eyr(s: &str) -> bool {
    match s.parse::<u32>() {
        Ok(yr) if yr >=2020 && yr <=2030  => true,
        _ => false,
    }
}
fn check_hgt(s: &str) -> bool {
    let end = &s[s.len()-2..];
    if end != "cm" && end != "in" {
        return false;
    }
    let val = s[..s.len()-2].parse::<u32>().unwrap();
    if end == "cm" && val <= 193 && val >= 150 {
        return true;
    }
    if end == "in" && val <= 76 && val >= 59 {
        return true;
    }
    false
}

fn check_hcl(s: &str) -> bool {
    if s.chars().next().unwrap() != '#' {
        return false;
    } 
    let vals = &s[1..];
    vals.chars().all(|c| "abcdef0123456798".contains(c))
}

fn check_ecl(s: &str) -> bool {
    s == "amb" || s == "blu" || s == "brn" || s == "gry" || s == "grn" || s == "hzl" || s == "oth"
}
fn check_pid(s: &str) -> bool {
    s.len() == 9 && s.chars().all(|c| char::is_digit(c, 10))
}

pub fn part1(input: String) {
    // ecl:amb pid:690616023 byr:1994 iyr:2014 hgt:172cm hcl:#c0946f eyr:2022 
    // eyr:1980 cid:97 hcl:z ecl:#102145 iyr:2011 byr:1945 pid:187cm hgt:179in 
    let mut valid: usize = 0;


    for l in input.lines() {
        let mut scores:HashMap<&str,&str> = HashMap::new();
        for pair in l.split(" ") {
            let mut s = pair.split(":");
            let n = s.next().unwrap();
            let v = s.next().unwrap();
            scores.insert(n,v);
        }
        if scores.contains_key("byr")  &&
            scores.contains_key("iyr") &&
            scores.contains_key("eyr") &&
            scores.contains_key("hgt") &&
            scores.contains_key("hcl") &&
            scores.contains_key("ecl") &&
            scores.contains_key("pid") //&&
            //scores.contains_key("cid")
        {
            valid = valid +1;
        }
    }
    println!("{}", valid);
}

pub fn part2(input: String) {
    // ecl:amb pid:690616023 byr:1994 iyr:2014 hgt:172cm hcl:#c0946f eyr:2022 
    // eyr:1980 cid:97 hcl:z ecl:#102145 iyr:2011 byr:1945 pid:187cm hgt:179in 
    let mut valid: usize = 0;


    for l in input.lines() {
        let mut scores:HashMap<&str,&str> = HashMap::new();
        for pair in l.split(" ") {
            let mut s = pair.split(":");
            let n = s.next().unwrap();
            let v = s.next().unwrap();
            scores.insert(n,v);
        }
        if scores.contains_key("byr") && check_byr(scores.get("byr").unwrap()) &&
            scores.contains_key("iyr") && check_iyr(scores.get("iyr").unwrap()) &&
            scores.contains_key("eyr") && check_eyr(scores.get("eyr").unwrap()) &&
            scores.contains_key("hgt") && check_hgt(scores.get("hgt").unwrap()) &&
            scores.contains_key("hcl") && check_hcl(scores.get("hcl").unwrap()) &&
            scores.contains_key("ecl") && check_ecl(scores.get("ecl").unwrap()) &&
            scores.contains_key("pid") && check_pid(scores.get("pid").unwrap())
            //scores.contains_key("cid")
        {
            valid = valid +1;
        }
    }
    println!("{}", valid);
}