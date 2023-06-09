use std::cmp::min;
use crate::shared::input;

fn str_to_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

pub fn part_one() {
    let input = input(8);
    let grid = str_to_grid(&input);
    let mut count = grid.len() * 2 + grid[0].len() * 2 - 4;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            let visible =
                grid[..=i - 1].iter().all(|row| row[j] < grid[i][j]) ||
                    grid[i + 1..].iter().all(|row| row[j] < grid[i][j]) ||
                    grid[i][..=j - 1].iter().all(|&e| e < grid[i][j]) ||
                    grid[i][j + 1..].iter().all(|&e| e < grid[i][j]);
            if visible {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn count_visible(iter: impl ExactSizeIterator<Item=u8>, value: u8) -> usize {
    min(
        iter.len(),
        iter.take_while(|&e| e < value).count() + 1,
    )
}

fn count_visible_col<'a>(iter: impl ExactSizeIterator<Item=&'a Vec<u8>>, col: usize, value: u8) -> usize {
    count_visible(iter.map(|row| row[col]), value)
}

pub fn part_two() {
    let input = input(8);
    let grid = str_to_grid(&input);
    let max = (1..grid.len() - 1)
        .flat_map(|i| (1..grid[i].len() - 1).map(|j| {
            let up = count_visible_col(grid[..i].iter().rev(), j, grid[i][j]);
            let down = count_visible_col(grid[i + 1..].iter(), j, grid[i][j]);
            let left = count_visible(grid[i][..j].iter().rev().copied(), grid[i][j]);
            let right = count_visible(grid[i][j + 1..].iter().copied(), grid[i][j]);
            up * down * left * right
        }).collect::<Vec<_>>()
        )
        .max()
        .unwrap();
    println!("{max:?}");
}

#[cfg(test)]
mod test {
    #[test]
    fn part_one() {
        // 1782
        super::part_one()
    }

    #[test]
    fn part_two() {
        // 474606
        super::part_two()
    }
}

const EXAMPLE: &str = "30373
25512
65332
33549
35390";