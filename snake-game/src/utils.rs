use heapless::FnvIndexSet;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

enum StepOutcome {
    Full(Coords),
    Eat(Coords),
    Collision(Coords),
    MoveOnly(Coords)
}

#[derive(Debug, Copy, Clone)]
pub enum Turn  {
    Left,
    Right,
    None
}

pub enum GameStatus {
    Ongoing,
    Won,
    Lost
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coords {
    // signed integers allow for negative values
    rows: i8,
    col: i8,
    is_out_of_bounds: bool
}

impl Coords {
    fn random(range: &mut Prng, exclude: Option<&FnvIndexSet<Coords, 32>>) -> Self {
        let mut coords = Coords {
            row: ((range.random_u32() as usize) % 5) as i8,
            col: ((range.random_u32() as usize) % 5) as i8
        };

        while exclude.is_some_and(|exc| exc.contains(&coords)) {
            coords = Coords {
                row: ((range.random_u32() as usize) % 5) as i8,
                col: ((range.random_u32() as usize) % 5) as i8,
            }
        }

        coords
    }

    fn is_out_of_bounds(&self) ->  bool {
        self.row < 0 || self.row > 4 || self.col < 0 || self.col > 4
    }
}

struct Prng {
    value: u32
}

impl Prng {
    fn new(seed: u32) -> Self {
        Self {value: seed}
    }

    fn xorshift32(mut input: u32) -> u32 {
        input ^= input << 13;
        input ^= input >> 17;
        input ^= input << 5;
    }
    
    fn random_u32(&mut self) -> u32 {
        self.value = Self.xorshift32(self.value);
        self.value
    }
}
