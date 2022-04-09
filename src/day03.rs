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
    // .#..#....##...#....#.....#.#...
    let nums: Vec<&[u8]> = input.lines()
        .map(|x| x.as_bytes())
        .collect();
    let r_len = nums[0].len();
    let mut counts: Vec<usize> = Vec::new();
    let slopes = [(1,1),(3,1),(5,1),(7,1),(1,2)];
    for (c_stride, r_stride) in slopes {
        let mut count = 0;
        // walk down by row stride
        let mut col = 0;
        let mut row = 0;

        while row < nums.len(){
            
            if nums[row][col%r_len] == '#' as u8 {
                count = count + 1;
            }
            col = col + c_stride;
            row = row + r_stride;
        }
        println!("{}", count);
        counts.push(count);
    }
    println!("{}", counts.iter().fold(1, |acc,&x| acc*x));

}