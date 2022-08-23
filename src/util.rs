pub fn wait_for_input() -> u8 {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}