use crate::shared::input;

pub fn solve() {
    let input = input(1);
    let max = input
        .split("\n\n")
        .map(|ei| {
            ei.split("\n")
                .map(|n| n.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();
    println!("{max}");
}

#[test]
fn part1() {
    // 69310
    solve()
}

pub fn solve2() {
    let input = input(1);
    let mut carryng = input
        .split("\n\n")
        .map(|ei| {
            ei.split("\n")
                .map(|n| n.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();
    carryng.sort();
    println!("{:?}", carryng.iter().rev().take(3).sum::<i32>());
}

#[test]
fn part2() {
    solve2()
}
