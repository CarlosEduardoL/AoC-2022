pub fn solve() {
    let input = include_str!("inputs\\day1.txt").trim_end();
    let max = input.split("\n\n").map(|ei| {
        ei.split("\n").map(|n| n.parse::<i32>().unwrap()).sum::<i32>()
    }).max().unwrap();
    println!("{max}");
}

#[test]
fn part1() {
    solve()
}

pub fn solve2() {
    let input = include_str!("inputs\\day1.txt").trim_end();
    let mut carryng = input.split("\n\n").map(|ei| {
        ei.split("\n").map(|n| n.parse::<i32>().unwrap()).sum::<i32>()
    }).collect::<Vec<_>>();
    carryng.sort();
    println!("{:?}", carryng.iter().rev().take(3).sum::<i32>());
}

#[test]
fn part2() {
    solve2()
}