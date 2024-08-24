mod utils;

use heapless::spsc::Queue;
use utils::{Coords, Direction};

pub struct Snake {
    head: Coords,
    tail: Queue<Coords, 32>,
    body_coords: FnvIndexSet<Coords, 32>,
    direction: Direction
}

impl Snake {
    fn new() -> Self {
        let head = Coords { row: 2, col: 2 };
        let initial_tail = Coords { row: 2, col: 1 };
        let mut tail = Queue::new();
        tail.enqueue(initial_tail).unwrap();

        let mut body_coords: FnvIndexSet<Coords, 32> = FnvIndexSet::new();
        body_coords.insert(head).unwrap();
        body_coords.insert(initial_tail).unwrap();

        Self { // this returns implicitly here.
            head,
            tail,
            body_coords,
            direction: Direction::Right
        }
    }

    fn move_snake(&mut self, to_coords: Coords, extend: bool) {
        self.tail.enqueue(self.head).unwrap();
        self.head = to_coords;
        self.body_coords.insert(to_coords).unwrap();

        if !extend {
            let back = self.tail.dequeue.unwrap();
            self.body_coords.remove(&back);
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
    }

    fn turn_right(&mut self) {
        self.direction  = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }

    fn make_turn(&mut self, direction: Direction) {
        match direction {
            Turn::Left => self.turn_left(),
            Turn::Right => self.turn_right(),
            Turn::None => () // do nothing
        }
    }
}
