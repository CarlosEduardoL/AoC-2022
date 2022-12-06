use crate::shared::input;

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }
    fn intersects(&self, other: &Range) -> bool {
        !(self.max < other.min || self.min > other.max)
    }
}

pub fn p1() {
    let result = input(4)
        .lines()
        .map(|l| {
            let mut ranges = l.split(",").map(|p| {
                let mut ids = p.split("-").map(|id| id.parse::<i32>().unwrap());
                Range {
                    min: ids.next().unwrap(),
                    max: ids.next().unwrap(),
                }
            });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .map(|(r1, r2)| (r1.contains(&r2) || r2.contains(&r1)) as i32)
        .sum::<i32>();
    println!("{result}");
}

#[test]
fn part1() {
    //441
    p1()
}

pub fn p2() {
    let result = input(4)
        .lines()
        .map(|l| {
            let mut ranges = l.split(",").map(|p| {
                let mut ids = p.split("-").map(|id| id.parse::<i32>().unwrap());
                Range {
                    min: ids.next().unwrap(),
                    max: ids.next().unwrap(),
                }
            });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .map(|(r1, r2)| r1.intersects(&r2) as i32)
        .sum::<i32>();
    println!("{result}");
}

#[test]
fn part2() {
    //861
    p2()
}
