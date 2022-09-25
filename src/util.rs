pub fn _wait_for_input() -> u8 {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

pub fn parse_action_trio(s: &str) -> (u8, u8, u8) {
    let split = s.split(",");
    let vec = split.collect::<Vec<&str>>();
    let mut u8vec = Vec::new();

    vec.iter().for_each(|x| u8vec.push(x.parse::<u8>().unwrap()));

    (u8vec[0], u8vec[1], u8vec[2])
}

pub fn serialize_action_trio(tup: (u8, u8, u8)) -> String {
    format!("{},{},{}", tup.0, tup.1, tup.2)
}