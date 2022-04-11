pub fn part1(input: String) {
    // azedybkmuwgotq
    // uqztdwasygmb
    let mut total: usize = 0;
    let mut my_answers: [u8;26] = [0;26];
    'l: for l in input.lines(){
        if l.len() == 0 {
            let mut my_total: usize = 0;
            for c in 0..26 {
                if my_answers[c] != 0 {
                    my_total = my_total +1;
                }
            }
            total = total + my_total;
            my_answers = [0;26];
            continue 'l;
        }

        for x in l.as_bytes().iter() {
            my_answers[(x-'a' as u8) as usize] = 1;
        }
    }
    let mut my_total: usize = 0;
    for c in 0..26 {
        if my_answers[c] != 0 {
            my_total = my_total +1;
        }
    }
    total = total + my_total;
    println!("{}", total);
}

pub fn part2(input: String) {
    // azedybkmuwgotq
    // uqztdwasygmb
    enum CountMode {
        Adding,
        Removing,
    }
    let mut total: usize = 0;
    let mut my_answers: [u8;26] = [0;26];
    let mut mode:CountMode = CountMode::Adding;

    'l: for l in input.lines(){
        if l.len() == 0 {
            let mut my_total: usize = 0;
            for c in 0..26 {
                if my_answers[c] != 0 {
                    my_total = my_total +1;
                }
            }
            total = total + my_total;
            my_answers = [0;26];
            mode = CountMode::Adding;
            continue 'l;
        }

        match mode {
            CountMode::Adding =>  {
                    for x in l.as_bytes().iter() {
                        my_answers[(x-'a' as u8) as usize] = 1;
                    }
                    mode = CountMode::Removing;
                },
            CountMode::Removing => {
                for x in 0..26 {
                    let c = (x + ('a' as u8)) as char;
                    if my_answers[x as usize] == 1 && !l.contains(c) {
                        my_answers[x as usize] = 0;
                    }
                }
            },
        }
    }
    let mut my_total: usize = 0;
    for c in 0..26 {
        if my_answers[c] != 0 {
            my_total = my_total +1;
        }
    }
    total = total + my_total;
    println!("{}", total);
}
