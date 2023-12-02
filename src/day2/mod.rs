pub fn part1() -> String {
    let path = "input/day2.txt";
    let input = std::fs::read_to_string(path).unwrap();
    input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if is_valid_line(line) {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
}

fn id_from_color(s: &str) -> usize {
    match s {
        "red" | "red," | "red;" => 0,
        "green" | "green," | "green;" => 1,
        "blue" | "blue," | "blue;" => 2,
        _ => panic!("Unknown color: {s}"),
    }
}

fn is_valid_line(line: &str) -> bool {
    let line = line.split_whitespace().collect::<Vec<_>>();
    let mut values = [0, 0, 0];
    for i in 0..line.len() / 2 {
        if i == 0 {
            continue;
        }
        let value = line[i * 2 + 1];
        let color = id_from_color(value);
        let value = line[i * 2];
        let value = value.parse::<u32>().unwrap();
        values[color] = std::cmp::max(value, values[color]);
    }
    values[0] <= 12 && values[1] <= 13 && values[2] <= 14
}

pub fn part2() -> String {
    let path = "input/day2.txt";
    let input = std::fs::read_to_string(path).unwrap();
    input.lines().map(minimum_cubes).sum::<usize>().to_string()
}

fn minimum_cubes(line: &str) -> usize {
    let line = line.split_whitespace().collect::<Vec<_>>();
    let mut values = [0, 0, 0];
    for i in 0..line.len() / 2 {
        if i == 0 {
            continue;
        }
        let value = line[i * 2 + 1];
        let color = id_from_color(value);
        let value = line[i * 2];
        let value = value.parse::<u32>().unwrap();
        values[color] = std::cmp::max(value, values[color]);
    }
    (values[0] * values[1] * values[2]) as usize
}
