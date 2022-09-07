use crate::pet::{BPet};
use crate::crew::Crew;

pub fn _headon_attack(my_attacker: &mut Option<BPet>, enemy_attacker: &mut Option<BPet>) {
    my_attacker.as_mut().unwrap().pet.health = my_attacker.as_ref().unwrap().pet.health
    - enemy_attacker.as_ref().unwrap().pet.power;
}

pub fn _check_life(pet: &mut Option<BPet>) -> bool {
    if pet.as_ref().unwrap().pet.health <= 0 {
        *pet = None;
        return true
    }
    false
}

pub fn _check_win_condition(my_crew: &Crew, enemy_crew: &Crew) -> u8 {
    let mut my_alive = 0;
    let mut enemy_alive = 0;
    for pet in &my_crew.friends {
        if pet.is_none() {
            continue;
        }
        if pet.as_ref().unwrap().pet.health > 0 {
            my_alive += 1;
        }
    }

    for pet in &enemy_crew.friends {
        if pet.is_none() {
            continue;
        }
        if pet.as_ref().unwrap().pet.health > 0 {
            enemy_alive += 1;
        }
    }

    if my_alive == 0 && enemy_alive == 0 {
        // Draw
        return 0;
    } else if my_alive > 0 && enemy_alive == 0 {
        // Win
        return 1;
    } else if my_alive == 0 && enemy_alive > 0  {
        // Lost
        return 2
    }
    // No conclusion
    return 3;
}