use heapless::FnvIndexSet;

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub enum StepOutcome {
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
pub struct Coords {
    // signed integers allow for negative values
    pub row: i8,
    pub col: i8,
}

impl Coords {
    pub fn random(range: &mut Prng, exclude: Option<&FnvIndexSet<Coords, 32>>) -> Self {
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

    pub fn is_out_of_bounds(&self) ->  bool {
        self.row < 0 || self.row > 4 || self.col < 0 || self.col > 4
    }
}

pub struct Prng {
    pub value: u32
}

impl Prng {
    pub fn new(seed: u32) -> Self {
        Self {value: seed}
    }

    fn xorshift32(&self, mut input: u32) -> u32 {
        input ^= input << 13;
        input ^= input >> 17;
        input ^= input << 5;

        input 
    }
    
    fn random_u32(&mut self) -> u32 {
        self.value = self.xorshift32(self.value);
        self.value
    }
}
