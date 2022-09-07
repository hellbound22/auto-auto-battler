use std::fmt;

use crate::food::Food;

#[derive(Clone, Debug, Default)]
pub struct SPet {
    pub pet: Pet,
    pub frozen: bool,
}

impl From<&Pet> for SPet {
    fn from(item: &Pet) -> Self {
        Self {
            pet: item.clone(),
            frozen: false,
        }
    }
}

impl From<SPet> for BPet {
    fn from(item: SPet) -> Self {
        Self {
            pet: item.pet.clone(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct BPet {
    pub pet: Pet,
    pub level: u8,
    pub xp: u8,
    pub food: Option<Food>,
    // TODO: Implement food
}


impl BPet {
    pub fn switch_food(&mut self, food: Food) {
        self.food = Some(food);
    }

    pub fn new_from_state_table(line: &str, ref_pet: &Pet) -> Self {
        let split = line.split(",");
        let vec = split.collect::<Vec<&str>>();
        BPet {
            pet: Pet::new_from_state_table(line, ref_pet),
            level: vec[3].parse().unwrap(),
            xp: vec[4].parse().unwrap(),
            food: None,
        }
    }
}

impl fmt::Display for BPet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}] {} ({}|{}) = {}/{}",
            self.pet.tier,
            self.pet.name,
            self.level,
            self.xp,
            self.pet.power,
            self.pet.health
        )
    }
}

impl Default for BPet {
    fn default() -> Self {
        Self {
            pet: Pet::default(),
            level: 1,
            xp: 0,
            food: None,
        }
    }
}

impl From<Pet> for BPet {
    fn from(item: Pet) -> Self {
        Self {
            pet: item,
            level: 1,
            xp: 0,
            food: None,
        }
    }
}

impl From<SPet> for Pet {
    fn from(item: SPet) -> Self {
        item.pet
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Pet {
    pub id: i8,
    pub tier: i8,
    pub name: String,
    pub power: i8,
    pub health: i8,
}

impl Pet {
    pub fn new(line: &str) -> Self {
        let split = line.split(",");
        let vec = split.collect::<Vec<&str>>();
        Pet {
            id: vec[0].parse().unwrap(),
            tier: vec[1].parse().unwrap(),
            name: vec[2].to_string(),
            power: vec[3].parse().unwrap(),
            health: vec[4].parse().unwrap(),
        }
    }

    pub fn new_from_state_table(line: &str, ref_pet: &Pet) -> Self {
        let split = line.split(",");
        let vec = split.collect::<Vec<&str>>();
        Pet {
            id: vec[0].parse().unwrap(),
            tier: ref_pet.tier,
            name: ref_pet.name.clone(),
            power: vec[1].parse().unwrap(),
            health: vec[2].parse().unwrap(),
        }
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
