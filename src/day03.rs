pub fn part1(input: String) {
    // .#..#....##...#....#.....#.#...
    let nums: Vec<&[u8]> = input.lines()
        .map(|x| x.as_bytes())
        .collect();
    let mut count = 0;
    let mut col = 0;
    for row in nums {
        if row[col%row.len()] == '#' as u8 {
            count = count + 1;
        }
        col = col +3;
    }
    println!("{}", count);
}

pub fn part2(input: String) {

}