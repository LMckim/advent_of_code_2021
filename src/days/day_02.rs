struct CMD {
    cmd: String,
    dist: i32,
}
struct Position {
    x: i32,
    y: i32,
}

fn split_input_to_cmds(input: &String) -> Vec<CMD> {
    return input
        .split('\n')
        .map(|x| x.split(' ').collect::<Vec<_>>())
        .map(|x| CMD {
            cmd: x[0].to_string(),
            dist: x[1].parse().unwrap(),
        })
        .collect();
}

pub fn day_02_solution_p1(input: &String) -> i32 {
    let mut pos = Position { x: 0, y: 0 };
    let cmds = split_input_to_cmds(&input);

    for cmd in cmds {
        match cmd.cmd.as_str() {
            "forward" => pos.x += cmd.dist,
            "down" => pos.y += cmd.dist,
            "up" => pos.y -= cmd.dist,
            _ => {}
        }
    }
    return pos.x * pos.y;
}

pub fn day_02_solution_p2(input: &String) -> i32 {
    let mut aim: i32 = 0;
    let mut pos = Position { x: 0, y: 0 };
    let cmds = split_input_to_cmds(&input);

    for cmd in cmds {
        match cmd.cmd.as_str() {
            "forward" => {
                pos.x += cmd.dist;
                pos.y += aim * cmd.dist
            }
            "down" => aim += cmd.dist,
            "up" => aim -= cmd.dist,
            _ => {}
        }
    }
    return pos.x * pos.y;
}
