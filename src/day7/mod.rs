use std::{
    char,
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub fn part1() -> String {
    let path = "input/day7.txt";
    let binding = std::fs::read_to_string(path).unwrap();
    let mut values = binding
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    values.sort_by(|a, b| compare_hands(a.0, b.0));

    values
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, v))| acc + (i + 1) * *v)
        .to_string()
}

fn card_value(a: char) -> u32 {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => a.to_digit(10).unwrap(),
    }
}

fn comapre_cards(a: char, b: char) -> Ordering {
    let a = card_value(a);
    let b = card_value(b);
    a.cmp(&b)
}

fn hand_points(hand: &str) -> u32 {
    let mut values = hand.chars().collect::<HashSet<char>>();
    let size = values.len();
    match size {
        1 => 6,
        2 => {
            let mut equal1 = 0;
            let first = values.drain().next().unwrap();
            for c in hand.chars() {
                if c == first {
                    equal1 += 1;
                }
            }
            if equal1 == 4 || equal1 == 1 {
                5
            } else {
                4
            }
        }
        3 => {
            let mut occurences = HashMap::new();
            for c in hand.chars() {
                let count = occurences.entry(c).or_insert(0);
                *count += 1;
            }
            for (k, v) in occurences {
                if v == 3 {
                    return 3;
                }
            }
            2
        }
        4 => 1,
        5 => 0,
        _ => panic!("Invalid hand"),
    }
}

fn compare_hands(a: &str, b: &str) -> Ordering {
    let points = hand_points(a).cmp(&hand_points(b));
    if points != Ordering::Equal {
        return points;
    }

    for (a, b) in a.chars().zip(b.chars()) {
        let points = comapre_cards(a, b);
        if points != Ordering::Equal {
            return points;
        }
    }

    Ordering::Equal
}

pub fn part2() -> String {
    let path = "input/day7.txt";
    let binding = std::fs::read_to_string(path).unwrap();
    let mut values = binding
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    values.sort_by(|a, b| compare_hands2(a.0, b.0));

    values
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, v))| acc + (i + 1) * *v)
        .to_string()
}

fn card_value2(a: char) -> u32 {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => a.to_digit(10).unwrap(),
    }
}

fn comapre_cards2(a: char, b: char) -> Ordering {
    let a = card_value2(a);
    let b = card_value2(b);
    a.cmp(&b)
}

fn hand_points2(hand: &str) -> u32 {
    let mut values = hand.chars().collect::<HashSet<char>>();
    let size = values.len();
    match size {
        1 => 6, // 5 of a kind
        2 => {
            let mut equal1 = 0;
            let first = values.drain().next().unwrap();
            for c in hand.chars() {
                if c == first {
                    equal1 += 1;
                }
            }
            if equal1 == 4 || equal1 == 1 {
                // 4 of a kind or 1 of a kind
                if hand.contains('J') {
                    return 6;
                }
                5
            } else {
                // Full house
                if hand.contains('J') {
                    6 // There are 2 J's
                } else {
                    4
                }
            }
        }
        3 => {
            let mut occurences = HashMap::new();
            for c in hand.chars() {
                let count = occurences.entry(c).or_insert(0);
                *count += 1;
            }
            for (k, v) in occurences.clone().iter() {
                if *v == 3 {
                    // 3 of a kind, so 2 are different
                    if hand.contains('J') {
                        // Only one J -> 4 of a kind
                        return 5;
                    }
                    return 3;
                }
            }
            match occurences.get(&'J') {
                // 2 J's and 1 pairs -> 4 of a kind
                Some(2) => 5,
                // 1 J's and 1 pairs -> 3 of a kind
                Some(1) => 4,
                // 2 pairs, no J's
                _ => 2,
            }
        }
        4 => {
            if hand.contains('J') {
                // 1 pair + 1 J or 2 J's -> 3 of a kind
                3
            } else {
                // 1 pairs
                1
            }
        }
        5 => {
            if hand.contains('J') {
                // 1 J -> 1 pair
                1
            } else {
                // 0 J's
                0
            }
        }
        _ => panic!("Invalid hand"),
    }
}

fn compare_hands2(a: &str, b: &str) -> Ordering {
    let points = hand_points2(a).cmp(&hand_points2(b));
    if points != Ordering::Equal {
        return points;
    }

    for (a, b) in a.chars().zip(b.chars()) {
        let points = comapre_cards2(a, b);
        if points != Ordering::Equal {
            return points;
        }
    }

    Ordering::Equal
}
