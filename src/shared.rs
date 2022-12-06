pub fn input(day: u8) -> String {
    std::fs::read_to_string(format!("src/inputs/day{day}.txt"))
        .unwrap()
        .trim_end()
        .into()
}
