#[derive(Copy, Clone)]
pub enum Automata {
    Redstone(u8),
    Water(u8),
    RedstoneBlock(),
    GameOfLife(bool),
    Air(),
    Slug(Direction),
    Slime(),
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn opposite(&self, other: Self) -> bool {
        match (self, other) {
            (Up, Down) => true,
            (Left, Right) => true,
            (Down, Up) => true,
            (Right, Left) => true,
            _ => false,
        }
    }
}

use self::Direction::*;

impl Automata {
    fn power_output(&self) -> u8 {
        match *self {
            Redstone(power) => power,
            RedstoneBlock() => 16,
            _ => 0,
        }
    }

    fn is_succeptible_to_liquid(&self) -> bool {
        match *self {
            Redstone(_) => true,
            RedstoneBlock() => true,
            Air() => true,
            Water(_) => true,
            _ => false,
        }
    }

    fn inflict(&self, other: Self, direction: Direction) -> Option<Self> {
        match &self {
            Water(wetness) => if *wetness != 0 && other.is_succeptible_to_liquid() {
                Some(Water((*wetness).max(1) - 1))
            } else {
                None
            },
            Slug(slug_direction) => if *slug_direction == direction {
                Some(Slug(*slug_direction))
            } else {
                None
            },
            _ => None,
        }
    }
}

use self::Automata::*;

pub struct Surroundings {
    pub topleft: Automata,
    pub topmiddle: Automata,
    pub topright: Automata,
    pub left: Automata,
    pub middle: Automata,
    pub right: Automata,
    pub bottomleft: Automata,
    pub bottommiddle: Automata,
    pub bottomright: Automata,
}

impl Surroundings {
    fn power_input(&self) -> u8 {
        self.topmiddle
            .power_output()
            .max(self.left.power_output())
            .max(self.right.power_output())
            .max(self.bottommiddle.power_output())
            .max(1) - 1
    }

    fn infliction_requested(&self) -> Option<Automata> {
        let middle = self.middle;
        self.topmiddle
            .inflict(middle, Down)
            .or(self.left.inflict(middle, Right))
            .or(self.right.inflict(middle, Left))
            .or(self.bottommiddle.inflict(middle, Up))
    }
}

pub fn next_middle(surroundings: Surroundings) -> Automata {
    if let Some(next) = surroundings.infliction_requested() {
        return next;
    }

    match surroundings.middle {
        Redstone(_) => Redstone(surroundings.power_input()),
        GameOfLife(false) => if surroundings.power_input() > 0 {
            GameOfLife(true)
        } else {
            GameOfLife(false)
        },
        Water(0) => Air(),
        Water(wetness) => Water(wetness - 1),
        Slug(_) => Slime(),
        a => a,
    }
}
