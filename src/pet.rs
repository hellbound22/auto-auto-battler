#[derive(Clone, Debug)]
pub struct BPet {
    pub pet: Pet,
    pub level: u8,
    pub xp: u8,
    // TODO: Implement food
}

impl Default for BPet {
    fn default() -> Self {
        Self {
            pet: Pet::default(),
            level: 1,
            xp: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pet {
    pub id: i8,
    pub tier: i8,
    pub name: String,
    pub power: i8,
    pub health: i8,
}

impl Pet {
    pub fn new(line: &str) -> Self {
        let mut split = line.split(",");
        let vec = split.collect::<Vec<&str>>();
        Pet { id: vec[0].parse().unwrap(), tier: vec[1].parse().unwrap(), name: vec[2].to_string(), power: vec[3].parse().unwrap(), health: vec[4].parse().unwrap() }
    }
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            id: 0,
            tier: 0,
            name: "Empty".to_owned(),
            power: 0,
            health: 0,
        }
    }
}