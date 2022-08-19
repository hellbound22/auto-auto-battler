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
    let mut bot = Game::new("std");

    //println!("{}", game);
    game.buy_pet(0, 0);
    bot.bot_random();
    //println!("\n\n==============BOT===============\n{}", bot);
    
    println!("======================BATTLE====================");
    game.battle(bot.crew.clone());
    //println!("{}", game);
    //println!("\n\n==============BOT===============\n{}", bot);
}
