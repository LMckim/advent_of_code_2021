use std::collections::HashMap;

fn split_input(input: &String) -> Vec<&str> {
    return input
        .split('\n')
        .collect();
}

fn most_common_bit(nums:&Vec<&str>, x_offset:usize) -> char {
    let mut cnts:HashMap<char, i32> = HashMap::new();
    cnts.entry('1').or_insert(0);
    cnts.entry('0').or_insert(0);
    for y in 0..nums.len() {
        let c:char  = nums[y].chars().nth(x_offset).unwrap();
        let upd = cnts.entry(c).or_insert(0);
        *upd += 1;
    }
    return if cnts[&'1'] >= cnts[&'0'] { '1' } else { '0' };
}

pub fn day_03_solution_p1(input: &String) -> i32 {
    let _bin_nums = split_input(&input);

    let mut gamma:String = "".to_string();
    let mut epsilon:String = "".to_string();
    for x in 0.._bin_nums[0].len() {
        if most_common_bit(&_bin_nums, x) == '1' {
            gamma += &"1";
            epsilon += &"0";
        } else {
            gamma += &"0";
            epsilon += &"1";
        }
    }
    return 
        i32::from_str_radix(&gamma, 2).unwrap()
        * i32::from_str_radix(&epsilon, 2).unwrap();
}

fn filter_nums(mut nums:Vec<&str>, eq:bool) -> Vec<&str> {
    let mut x  = 0;
    while nums.len() > 1 {
        let mc_bit = most_common_bit(&nums, x);
        nums = nums.iter().filter(|bin| {
            if eq {
                bin.chars().nth(x).unwrap() == mc_bit
            } else {
                bin.chars().nth(x).unwrap() != mc_bit
            }
        }).cloned().collect::<Vec<_>>();
        x += 1;

    }
    return nums;
}

pub fn day_03_solution_p2(input: &String) -> i32 {
    let _bin_nums = split_input(&input);
    let oxy_nums = filter_nums(_bin_nums.clone(), true);
    let co2_nums = filter_nums(_bin_nums.clone(), false);

    return 
        i32::from_str_radix(oxy_nums[0], 2).unwrap()
        * i32::from_str_radix(co2_nums[0], 2).unwrap();
}