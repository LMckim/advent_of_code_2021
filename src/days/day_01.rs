fn split_input(input: &String) -> Vec<i32> {
    return input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

pub fn day_01_solution_p1(input: &String) -> i32 {
    let nums: Vec<i32> = split_input(&input);
    let mut cnt: i32 = 0;
    for (i, el) in nums.iter().enumerate() {
        if i != 0 && el > &nums[i - 1] {
            cnt += 1;
        }
    }
    return cnt;
}

#[allow(unused_assignments)]
pub fn day_01_solution_p2(input: &String) -> i32 {
    let nums: Vec<i32> = split_input(&input);

    let mut cnt: i32 = 0;
    let mut prev_sum: i32 = 0;
    for i in 2..nums.len() {
        let s = nums[i] + nums[i - 1] + nums[i - 2];
        if prev_sum == 0 {
            prev_sum = s;
        } else if s > prev_sum {
            cnt += 1;
        }
        prev_sum = s;
    }
    return cnt;
}
