mod days;

fn main() {
    let day_01_input = std::fs::read_to_string("./inputs/day_01.in").unwrap();
    let day_01_p1 = days::day_01::day_01_solution_p1(&day_01_input);
    println!("Day 01 Part 01 Solution:\t{}", day_01_p1);
    let day_01_p2 = days::day_01::day_01_solution_p2(&day_01_input);
    println!("Day 01 Part 02 Solution:\t{}", day_01_p2);

    let day_02_input = std::fs::read_to_string("./inputs/day_02.in").unwrap();
    let day_02_p1 = days::day_02::day_02_solution_p1(&day_02_input);
    println!("Day 02 Part 01 Solution:\t{}", day_02_p1);
    let day_02_p2 = days::day_02::day_02_solution_p2(&day_02_input);
    println!("Day 02 Part 02 Solution:\t{}", day_02_p2);
}
