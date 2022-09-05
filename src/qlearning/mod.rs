use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use crate::pet::BPet;
use crate::pet::Pet;
use crate::food::Food;

pub struct StateTable {
    map: HashMap<Option<BPet>, usize>,
    pet_bucket: HashMap<u8, Pet>,
    buffer: String,
}

impl StateTable {
    pub fn new() -> Self {
        unimplemented!()
    }

}