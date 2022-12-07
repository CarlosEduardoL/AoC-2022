use crate::shared::input;

fn get_start_index(input: &str, prelude_len: usize) -> usize {
    let input = input
        .as_bytes()
        .iter()
        .map(|c| *c as char)
        .collect::<Vec<_>>();
    for i in 0..(input.len() - (prelude_len + 1)) {
        let set = input[i..i + prelude_len]
            .iter()
            .collect::<std::collections::HashSet<_>>();
        //println!("{set:?}");
        if set.len() == prelude_len {
            return i + prelude_len;
        }
    }
    0
}

pub fn part_one() {
    let input = input(6);
    println!("{}", get_start_index(&input, 4));
}

pub fn part_two() {
    let input = input(6);
    println!("{}", get_start_index(&input, 14));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        // 1658
        super::part_one();
    }
    #[test]
    fn part_two() {
        // 2260
        super::part_two();
    }
}
