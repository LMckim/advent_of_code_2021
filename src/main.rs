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

    let day_03_input = std::fs::read_to_string("./inputs/day_03.in").unwrap();
    let day_03_p1 = days::day_03::day_03_solution_p1(&day_03_input);
    println!("Day 03 Part 01 Solution:\t{}", day_03_p1);
    let day_03_p2 = days::day_03::day_03_solution_p2(&day_03_input);
    println!("Day 03 Part 02 Solution:\t{}", day_03_p2);

    let day_04_input = std::fs::read_to_string("./inputs/day_04.in").unwrap();
    let day_04_p1 = days::day_04::day_04_solution_p1(&day_04_input);
    println!("Day 04 Part 01 Solution:\t{}", day_04_p1);
    let day_04_p2 = days::day_04::day_04_solution_p2(&day_04_input);
    println!("Day 04 Part 02 Solution:\t{}", day_04_p2);
}
