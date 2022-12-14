use Options::*;
use Results::*;

use crate::shared::input;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Results {
    LOST = 0,
    DRAW = 3,
    WIN = 6,
}

impl Results {
    fn from(enc: u8) -> Results {
        match enc {
            b'X' => LOST,
            b'Y' => DRAW,
            _ => WIN,
        }
    }

    fn from_opt(you: Options, other: Options) -> Results {
        match (other, you) {
            (Rock, Paper) => WIN,
            (Rock, Scissors) => LOST,
            (Paper, Rock) => LOST,
            (Paper, Scissors) => WIN,
            (Scissors, Rock) => WIN,
            (Scissors, Paper) => LOST,
            _ => DRAW,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Options {
    Rock = 1,
    Paper,
    Scissors,
}

impl Options {
    fn rps(self, yours: Self) -> i32 {
        yours as i32 + Results::from_opt(yours, self) as i32
    }

    fn from(enc: u8) -> Options {
        match enc {
            b'A' | b'X' => Options::Rock,
            b'B' | b'Y' => Options::Paper,
            b'C' | b'Z' => Options::Scissors,
            _ => unreachable!("Tis will never happen"),
        }
    }

    fn from_res(self, res: Results) -> Options {
        unsafe {
            std::mem::transmute(match res {
                LOST => {
                    if self as u8 - 1 > 0 {
                        self as u8 - 1
                    } else {
                        3
                    }
                }
                DRAW => self as u8,
                WIN => {
                    if self as u8 + 1 < 4 {
                        self as u8 + 1
                    } else {
                        1
                    }
                }
            })
        }
    }
}

pub fn p1() {
    // 11841
    let res = input(2)
        .lines()
        .map(|l| l.as_bytes())
        .map(|array| (Options::from(array[0]), Options::from(array[2])))
        .map(|(e, y)| e.rps(y))
        .sum::<i32>();
    println!("{res}");
}

#[test]
fn part1() {
    p1()
}

pub fn p2() {
    // 13022
    let res = input(2)
        .lines()
        .map(|l| l.as_bytes())
        .map(|array| (Options::from(array[0]), Results::from(array[2])))
        .map(|(e, y)| e.rps(e.from_res(y)))
        .sum::<i32>();
    println!("{res}");
}

#[test]
fn part2() {
    p2()
}
