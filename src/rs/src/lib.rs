
extern crate rand;

pub mod game;
pub mod snake;
mod util;

use self::game::Game;

#[no_mangle]
pub fn new(w: u16, h: u16) -> Game {
	Game::new(w, h)
}
