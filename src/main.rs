mod store;
mod pet;
mod crew;
mod game;

use store::Store;
use pet::{BPet, Pet};
use crew::Crew;
use game::Game;


fn main() {
    let mut game = Game::new("std");

    println!("{}", game);
    //dbg!(game.crew);
}
