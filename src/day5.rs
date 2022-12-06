use crate::shared::input;

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

pub fn part_one() {
    let input = input(5);
    let mut input = input.split("\n\n");

    let mut diagram = input.next().unwrap().lines().rev();
    let instructions = input
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
        .map(|arr| Instruction {
            count: arr[1].parse().unwrap(),
            from: arr[3].parse().unwrap(),
            to: arr[5].parse().unwrap(),
        })
        .collect::<Vec<_>>();

    let mut stacks = diagram
        .next()
        .unwrap()
        .split(" ")
        .filter(|c| !c.is_empty())
        .map(|_| std::collections::LinkedList::<char>::new())
        .collect::<Vec<_>>();

    for line in diagram {
        let pices = line
            .chars()
            .enumerate()
            .filter(|(i, _)| *i > 0 && (*i - 1) % 4 == 0)
            .map(|(_, c)| c)
            .collect::<Vec<_>>();
        for (i, c) in pices.iter().enumerate() {
            if *c != ' ' {
                stacks[i].push_back(*c);
            }
        }
    }

    for instruction in instructions {
        for _ in 0..instruction.count {
            let tmp = stacks[instruction.from - 1].pop_back().unwrap();
            stacks[instruction.to - 1].push_back(tmp)
        }
    }

    let result = stacks.iter().map(|s| s.back().unwrap()).collect::<String>();
    println!("{result}");
}

#[test]
fn test_one() {
    part_one();
}

pub fn part_two() {
    let input = input(5);
    let mut input = input.split("\n\n");

    let mut diagram = input.next().unwrap().lines().rev();
    let instructions = input
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
        .map(|arr| Instruction {
            count: arr[1].parse().unwrap(),
            from: arr[3].parse().unwrap(),
            to: arr[5].parse().unwrap(),
        })
        .collect::<Vec<_>>();

    let mut stacks = diagram
        .next()
        .unwrap()
        .split(" ")
        .filter(|c| !c.is_empty())
        .map(|_| Vec::<char>::new())
        .collect::<Vec<_>>();

    for line in diagram {
        let pices = line
            .chars()
            .enumerate()
            .filter(|(i, _)| *i > 0 && (*i - 1) % 4 == 0)
            .map(|(_, c)| c)
            .collect::<Vec<_>>();
        for (i, c) in pices.iter().enumerate() {
            if *c != ' ' {
                stacks[i].push(*c);
            }
        }
    }

    for instruction in instructions {
        let len = stacks[instruction.from - 1].len();
        let mut copy = stacks[instruction.from - 1][len-instruction.count..len].iter().map(|c|*c).collect::<Vec<_>>();
        stacks[instruction.to - 1].append(&mut copy);
        for _ in 0..instruction.count {
            stacks[instruction.from - 1].pop();
        }
    }

    let result = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    println!("{result}");
}

#[test]
fn test_two() {
    part_two();
}