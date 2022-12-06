use std::collections::HashSet;

use crate::shared::input;

pub fn p1() {
    let result = input(3)
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(p1, p2)| {
            **p1.as_bytes()
                .iter()
                .collect::<HashSet<_>>()
                .intersection(&p2.as_bytes().iter().collect::<HashSet<_>>())
                .next()
                .unwrap()
        })
        .map(|item| match item {
            b'a'..=b'z' => item - b'a' + 1,
            _ => item - b'A' + 27,
        } as i32)
        .sum::<i32>();
    println!("{result}");
}

#[test]
fn part1() {
    // 7878
    p1()
}

pub fn p2() {
    let result = input(3)
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|e| {
            *e.iter()
                .map(|s| s.chars().collect::<HashSet<_>>())
                .fold(HashSet::new(), |acc, hs| {
                    if acc.len() == 0 {
                        hs
                    } else {
                        acc.intersection(&hs).cloned().collect()
                    }
                })
                .iter()
                .next()
                .unwrap() as u8
        })
        .map(|item| match item {
            b'a'..=b'z' => item - b'a' + 1,
            _ => item - b'A' + 27,
        } as i32)
        .sum::<i32>();
    println!("{result}");
}

#[test]
fn part2() {
    // 7878
    p2()
}
