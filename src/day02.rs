pub fn part1(input: String) {
    // 2-4 d: mvdclzddj
    let raw_lines: Vec<&str> = input.lines().collect();
    let mut total = 0;
    for line in raw_lines {
        let mut split = line.split(' ');
        let (r, l, s) = (split.next().unwrap(), 
                                            split.next().unwrap().chars().nth(0).unwrap(),
                                            split.next().unwrap_or_else(|| panic!("{}", line)));
        let mut rsplit = r.split('-');
        let r1 = rsplit.next().unwrap().parse::<usize>().unwrap();
        let r2 = rsplit.next().unwrap().parse::<usize>().unwrap();
        let c = s.matches(l).count();
        if c >= r1 && c <= r2 {
            total = total +1;
        } else {
            total = total +0;
        }

    }
    println!("{}", total);
}

pub fn part2(input: String) {
    // 2-4 d: mvdclzddj
    let raw_lines: Vec<&str> = input.lines().collect();
    let mut total = 0;
    for line in raw_lines {
        let mut split = line.split(' ');
        let (r, l, s) = (split.next().unwrap(), 
                                            split.next().unwrap().chars().nth(0).unwrap(),
                                            split.next().unwrap_or_else(|| panic!("{}", line)));
        let mut rsplit = r.split('-');
        let r1 = rsplit.next().unwrap().parse::<usize>().unwrap();
        let r2 = rsplit.next().unwrap().parse::<usize>().unwrap();
        if s.chars().nth(r1-1).unwrap() == l 
        && s.chars().nth(r2-1).unwrap_or_else(|| panic!("{}", line)) != l {
            total = total +1;
        } else if s.chars().nth(r1-1).unwrap() != l 
        && s.chars().nth(r2-1).unwrap() == l {
            total = total +1;
        } else{
            total = total +0;
        }
    }
    println!("{}", total);
}
