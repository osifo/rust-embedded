pub mod utils;
mod snake;

use heapless::FnvIndexSet;
use utils::{Coords, GameStatus, Prng, Direction, StepOutcome, Turn};
use snake::Snake;


pub(crate) struct Game {
    rng: Prng,
    snake: Snake,
    food_coords: Coords,
    speed: u8,
    pub(crate) status: GameStatus,
    score: u8
}

impl Game {
    pub(crate) fn new(rng_seed: u32) -> Self {
        let mut rng = Prng::new(rng_seed);

        // TODO -  i don't think this tail logic here is needed.
        let mut tail: FnvIndexSet<Coords, 32> =  FnvIndexSet::new();
        tail.insert(Coords{row: 2, col: 1}).unwrap();

        let snake = Snake::new();
        let food_coords = Coords::random(&mut rng, Option::Some(&snake.body_coords));

        Self {
            rng,
            snake,
            food_coords,
            speed: 1,
            status: GameStatus::Ongoing,
            score: 0
        }
    }

    //function to reset and restart the game
    pub(crate) fn reset(&mut self) {
        self.snake = Snake::new();
        self.place_food();
        self.speed = 1;
        self.score = 0;
        self.status = GameStatus::Ongoing;
    }

    fn place_food(&mut self) -> Coords {
        let coords = Coords::random(&mut self.rng, Option::Some(&self.snake.body_coords));
        self.food_coords = coords;
        coords
    }

    fn wraparound(&self, next_head_coords: Coords) -> Coords {
        if next_head_coords.row < 0 {
            Coords { row: 4, ..next_head_coords }
        } else if next_head_coords.row > 4 {
            Coords { row: 0, ..next_head_coords }
        } else if next_head_coords.col < 0 {
            Coords { col: 4, ..next_head_coords }
        } else {
            Coords { col: 0, ..next_head_coords }
        }
    }

    fn get_next_move(&self) -> Coords {
        let head = &self.snake.head;

        let next_move = match self.snake.direction {
            Direction::Up => Coords { row: head.row - 1, col: head.col },
            Direction::Down => Coords { row: head.row + 1, col: head.col },
            Direction::Left => Coords { row: head.row, col: head.col - 1 },
            Direction::Right => Coords { row: head.row, col: head.col + 1 },
        };

        if next_move.is_out_of_bounds() {
            self.wraparound(next_move)
        } else {
            next_move
        }
    }

    fn process_next_move(&self) -> StepOutcome {
        let next_move = self.get_next_move();
        
        if self.snake.body_coords.contains(&next_move) {
            if next_move == *self.snake.tail.peek().unwrap() {
                // this means that the next move would be the immediate former tail, so no collision
                StepOutcome::MoveOnly(next_move)
            } else {
                StepOutcome::Collision(next_move)
            }
        } else if next_move == self.food_coords {
            if self.snake.tail.len() == 23 { // the slack has reached it's max growth
                StepOutcome::Full(next_move)
            } else {
                StepOutcome::Eat(next_move)
            }
        } else {
            StepOutcome::MoveOnly(next_move)
        }
    }

    fn execute_move(&mut self, next_move_outcome: StepOutcome) {
        self.status = match next_move_outcome {
            StepOutcome::Full(_) => GameStatus::Won,
            StepOutcome::Collision(_) => GameStatus::Lost,
            StepOutcome::Eat(coords) => {
                self.snake.move_snake(coords, true);
                self.score += 1;

                if self.score % 5 == 0 {
                    self.speed += 1
                }
                self.place_food();

                GameStatus::Ongoing
            },
            StepOutcome::MoveOnly(coords) => {
                self.snake.move_snake(coords, false);
                GameStatus::Ongoing
            }
        }
    }

    pub(crate) fn step(&mut self, turn: Turn) {
        self.snake.make_turn(turn);
        let next_move =  self.process_next_move();
        self.execute_move(next_move);
    }

    pub(crate) fn calc_step_interval(&self) -> u32 {
        let interval = 1000 - (((self.speed as i32) - 1) * 200);

        if interval < 200 { 
            200u32
        } else {
            interval as u32
        }
    }

    pub(crate) fn game_matrix(
        &self,
        head_brightness: u8,
        tail_brightness: u8,
        food_brightness: u8,
    ) -> [[u8; 5]; 5] {
        let mut board_leds = [[0u8; 5]; 5];
        board_leds[self.snake.head.row as usize][self.snake.head.col as usize] = head_brightness;
        for cell in &self.snake.tail {
            board_leds[cell.row as usize][cell.col as usize] = tail_brightness;
        }
        board_leds[self.food_coords.row as usize][self.food_coords.col as usize] = food_brightness;

        board_leds
    }

    pub(crate) fn score_matrix(&self) -> [[u8; 5]; 5] {
        let mut board_leds = [[0u8; 5]; 5];
        let score_rows =  (self.score as usize) / 5;
        let score_cols = (self.score as usize) % 5;

        for row in 0..score_rows {
            board_leds[row] = [1; 5];
        }

        for col in 0..score_cols {
            board_leds[score_rows][col] = 1;
        }

        board_leds
    }
}