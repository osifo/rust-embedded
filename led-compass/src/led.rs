#[derive(Debug)]

pub enum Direction {
  North,
  West,
  South,
  East,
  NorthWest,
  SouthWest,
  SouthEast,
  NorthEast,
}

// these nested arrays set the patter for which the led should be lit up

const NORTH: [[u8; 5]; 5] = [
  [0, 0, 1, 0, 0],
  [0, 1, 0, 1, 0],
  [1, 0, 1, 0, 1],
  [0, 0, 1, 0, 0],
  [0, 0, 1, 0, 0]
];

const WEST: [[u8; 5]; 5] = [
  [0, 0, 1, 0, 0],
  [0, 0, 0, 1, 0],
  [1, 1, 1, 1, 1],
  [0, 0, 0, 1, 0],
  [0, 0, 1, 0, 0]
];

const SOUTH: [[u8; 5]; 5] = [
  [0, 0, 1, 0, 0],
  [0, 0, 1, 0, 0],
  [1, 1, 1, 1, 1],
  [0, 1, 0, 1, 0],
  [0, 0, 1, 0, 0]
];

const EAST: [[u8; 5]; 5] = [
  [0, 0, 1, 0, 0],
  [0, 1, 1, 0, 0],
  [1, 0, 1, 1, 1],
  [0, 1, 1, 0, 0],
  [0, 0, 1, 0, 0]
];

const NORTH_WEST: [[u8; 5]; 5] = [
  [0, 0, 1, 1, 1],
  [0, 0, 0, 1, 1],
  [0, 0, 1, 0, 1],
  [0, 1, 0, 0, 0],
  [1, 0, 0, 0, 0]
];

const SOUTH_WEST: [[u8; 5]; 5] = [
  [1, 0, 0, 0, 0],
  [0, 1, 0, 0, 0],
  [0, 0, 1, 0, 1],
  [0, 0, 0, 1, 1],
  [0, 0, 1, 1, 1]
];

const SOUTH_EAST: [[u8; 5]; 5] = [
  [0, 0, 0, 0, 1],
  [1, 0, 0, 1, 0],
  [1, 1, 1, 0, 0],
  [1, 0, 1, 0, 0],
  [1, 1, 1, 1, 0]
];

const NORTH_EAST: [[u8; 5]; 5] = [
  [1, 1, 1, 1, 0],
  [1, 1, 1, 0, 0],
  [1, 1, 1, 0, 0],
  [1, 0, 0, 1, 0],
  [0, 0, 0, 0, 1]
];


pub fn direction_to_led(direction: Direction) -> [[u8; 5]; 5] {
  match direction {
    Direction::North => NORTH,
    Direction::West => WEST,
    Direction::South => SOUTH,
    Direction::East => EAST,
    Direction::NorthWest => NORTH_WEST,
    Direction::SouthWest => SOUTH_WEST,
    Direction::SouthEast => SOUTH_EAST,
    Direction::NorthEast => NORTH_EAST,
  }
}