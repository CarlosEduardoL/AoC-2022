use std::collections::HashMap;
use crate::shared::input;

fn get_sizes() -> Vec<usize> {
    let input: String = input(7);
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut path = Vec::new();
    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["$", "cd", "/"] => path.push("/".to_string()),
            ["$", "cd", ".."] => { path.pop(); }
            ["$", "cd", d] => {
                let last = path.last().unwrap();
                path.push(format!("{}{}{}", last, if *last != "/" { "/" } else { "" }, d));
            }
            [size, _] => {
                if let Ok(size) = size.parse::<usize>() {
                    for p in &path {
                        *sizes.entry(p.to_string()).or_insert(0) += size;
                    }
                }
            }
            _ => {}
        }
    }
    sizes.values().map(|x| *x).collect()
}

pub fn part_one() {
    println!("{}", get_sizes().iter().filter(|&&s| s <= 100_000).sum::<usize>())
}

pub fn part_two() {
    let sizes = get_sizes();
    println!("{}", sizes.iter().filter(|&&s| s >= 30_000_000 - (70_000_000 - sizes.iter().max().unwrap())).min().unwrap())
}

#[cfg(test)]
mod test {
    #[test]
    fn part_one() {
        // 1350966
        super::part_one()
    }

    #[test]
    fn part_two() {
        // 6296435
        super::part_two()
    }
}