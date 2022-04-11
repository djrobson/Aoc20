use std::collections::HashMap;

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
        if scores.contains_key("byr") && 
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
    // .#..#....##...#....#.....#.#...
    ()

}