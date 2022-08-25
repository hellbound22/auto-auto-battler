mod store;
mod pet;
mod crew;
mod game;
mod battle;
mod util;
mod food;
mod qlearning;

use game::Game;


fn main() {
    let mut game = Game::new("std");
    qlearning::StateTable::new("./qtables/std/state.table", game.get_buckets());
    
    std::process::exit(0);

    
    let mut bot = Game::new("std");

    game.bot_random();
    bot.bot_random();
    
    game.game_loop(bot.crew.clone());
}
