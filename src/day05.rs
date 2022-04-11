pub fn part1(input: String) {
    let mut max: usize = 0;
    for l in input.lines() {
        let b = l.as_bytes();
        let mut this: usize = 0;
        let len = 9;
        for o in 0..=len {
            let my_o = len -o;
            if b[my_o] == 'B' as u8 || b[my_o] == 'R' as u8 {
                this = this + (1<<o);
            }
        }
        if this > max {
            max = this;
        }
    }
    println!("{}", max);
}

pub fn part2(input: String) {
  ()
}