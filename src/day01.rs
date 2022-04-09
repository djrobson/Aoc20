pub fn part1(input: String) {
    //println!("{}", input.chars().rev().collect::<String>());
    let nums: Vec<u32> = input.lines()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();
    for x in 0..nums.len() {

        for y in x+1..nums.len() {
            let this_num = nums[x];
            if 2020 == this_num + nums[y] {
                println!("{}",this_num * nums[y]);
                return();
            }
        }
    }
    println!("didn't find it");
}

pub fn part2(input: String) {
    let mut nums: Vec<u32> = input.lines()
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect::<Vec::<u32>>();
    nums.sort();

    for x in 0..nums.len() {
        'y: for y in x+1..nums.len() {
            if nums[x] + nums[y] > 2020 {
                continue 'y;
            }
            for z in y+1..nums.len() {
                if nums[x] + nums[y] + nums[z] == 2020 {
                    println!("{} {} {} = {}", 
                        nums[x], nums[y], nums[z], 
                        nums[x]*nums[y]*nums[z]);
                    return;
                }
            }
        }
    }
    println!("didn't find it");



}
