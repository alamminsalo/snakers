extern crate rand;
use rand::{Rng};

use snake::{Snake, Direction};
use util;

#[derive(PartialEq)]
pub struct Game {
    w: u16,
    h: u16,
    snakes: Vec<Snake>,
    food: Option<(i16,i16)>,
    ended: bool
}

impl Game {

    pub fn width(&self) -> u16 {
      self.w
    }

    pub fn height(&self) -> u16 {
      self.h
    }

    pub fn get_snake(&mut self) -> &mut Snake {
      &mut self.snakes[0]
    }

    pub fn get_food(&self) -> &Option<(i16,i16)> {
      &self.food
    }

    pub fn is_ended(&self) -> bool {
      self.ended
    }

    // Creates a new game
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            w: width,
            h: height,
            snakes: vec![Snake::new()],
            food: None,
	    ended: false
        }
    }

    // Handle border-crossing and translates coordinates when needed
    fn translate(dim: (u16, u16), pos: (i16,i16)) -> (i16,i16) {
        (util::wrap(0, pos.0, dim.0 as i16 - 1), util::wrap(0, pos.1, dim.1 as i16 - 1))
    }

    pub fn reset(&mut self, w: u16, h: u16) {
    	self.w = w;
	self.h = h;
        self.snakes = vec![Snake::new()];
	self.food = None;
	self.ended = false;
    }

    // Ticks game state
    pub fn tick(&mut self) {
	if !self.ended {
	  let dim = (self.w, self.h);
	  for snake in self.snakes.iter_mut() {
	      let peeked = snake.peek();
	      let translated = Game::translate(dim, peeked);

	      let h0 = snake.head();
	      let d0 = snake.d();

	      let head = snake.goto(translated);
	      let d1 = snake.d();

	      // Hitting self kills the snake
	      if snake.body()[1..].contains(&peeked) {
		self.ended = true;
	      }

	      // Food grows snake
	      if self.food != None && head == self.food.unwrap() {
		  snake.grow();

		  // Remove food
		  self.food = None;
	      }
	  }
	  if self.food == None {
		  self.add_food();
	  }
	}
    }

    fn grid(&self) -> Vec<(i16,i16)> {
        let mut g = vec![];
        for y in 0..self.h as i16 {
            for x in 0..self.w as i16 {
                g.push((x, y));
            }
        }
        g
    }

    fn free_grid(&self) ->Vec<(i16,i16)> {
        let mut grid = self.grid();
        for snake in self.snakes.iter() {
            grid.retain(|x: &(i16,i16)| !snake.body().contains(x));
        }
	grid
    }

    fn add_food(&mut self) {
    	let grid = self.free_grid();
        // Grid now contains only positions that are free
        self.food = Some(rand::thread_rng().choose(&grid).unwrap().clone());
    }
}

#[cfg(test)]
mod tests {
#[test]
fn test_tick() {
    let mut game = Game::new(32, 32);

    {
        let snake = game.snakes.get_mut(0).unwrap();
        snake.goto((0,0));
        snake.dir(Direction::Left);
    }

    game.tick();
    game.tick();
    game.tick();

    {
        let snake = game.snakes.get_mut(0).unwrap();
        assert_eq!((29, 0), snake.head());
        snake.dir(Direction::Right);
    }

    game.tick();
    game.tick();

    {
        let snake = game.snakes.get_mut(0).unwrap();
        assert_eq!((27, 0), snake.head());
    }
}

#[test]
fn test_grid() {
	let mut game = Game::new(10,10);
    {
        let snake = game.snakes.get_mut(0).unwrap();
        snake.goto((0,0));
        snake.dir(Direction::Left);
    }

    game.tick();
    game.tick();

	let grid = game.grid();
    let free = game.free_grid();
    assert!(free.len() < game.grid().len());

    {
        let snake = game.snakes.get_mut(0).unwrap();
	assert!(free.len() + snake.body().len() == grid.len());
	for part in &snake.body() {
		assert!(!free.contains(&part));
	}
    }
}
}
