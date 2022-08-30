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
    let states = qlearning::StateTable::new("./qtables/std/meta_state.table", "./qtables/std/meta_state.table", game.get_buckets());
    game.bot_random();

    //dbg!(states.get_state_index(game.get_state()));

    
    std::process::exit(0);

    
    let mut bot = Game::new("std");

    game.bot_random();
    bot.bot_random();
    
    game.game_loop(bot.crew.clone());
}
