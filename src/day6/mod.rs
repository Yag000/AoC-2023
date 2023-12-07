pub fn part1() -> String {
    let path = "input/day6.txt";
    let values = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(':').collect::<Vec<&str>>()[1]
                .split(' ')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let number = values[0].len();
    let mut value = 1;
    for i in 0..number {
        value *= (1..(values[0][i]))
            .filter_map(|v| {
                if v * (values[0][i] - v) > values[1][i] {
                    Some(v)
                } else {
                    None
                }
            })
            .count();
    }
    value.to_string()
}

pub fn part2() -> String {
    let path = "input/day6.txt";
    let values = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(':').collect::<Vec<&str>>()[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .fold(String::new(), |acc, s| format!("{}{}", acc, s))
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();
    (0..values[0])
        .filter(|v| v * (values[0] - v) > values[1])
        .count()
        .to_string()
}
