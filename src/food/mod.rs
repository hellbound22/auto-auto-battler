#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Food {
    pub id: i8,
    pub tier: i8,
    pub name: String,
    pub frozen: bool,
    pub type_effect: u8,
    /*  Types of effects: 
        0 > Pure stats, targted
        1 > Pure stats, team wide
        2 > Pure stats, random friends
    */

}

impl Food {
    pub fn new(line: &str) -> Self {
        let split = line.split(",");
        let vec = split.collect::<Vec<&str>>();
        Food {
            id: vec[0].parse().unwrap(),
            tier: vec[1].parse().unwrap(),
            name: vec[2].to_string(),
            frozen: false,
            type_effect: vec[3].parse().unwrap(),
        }
    }
}
