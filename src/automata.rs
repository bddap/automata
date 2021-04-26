#[derive(Debug, Copy, Clone)]
pub enum Automata {
    Redstone(u8),
    Water(u8),
    RedstoneBlock(),
    GameOfLife(bool),
    Air(),
    Slug(Direction),
    Slime(),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

use self::Direction::*;

impl Automata {
    fn is_succeptible_to_liquid(&self) -> bool {
        match *self {
            Redstone(_) => true,
            RedstoneBlock() => true,
            Air() => true,
            Water(_) => true,
            _ => false,
        }
    }

    fn wet(&self, wetness: u8) -> Option<Self> {
        // return what self becomes when it gets wet
        // None if the water has no effect
        assert!(wetness != 0);
        let wetness_here = wetness.max(1) - 1;
        let maybe = Some(Water(wetness_here));
        match *self {
            Redstone(_) => maybe,
            RedstoneBlock() => maybe,
            Air() => maybe,
            Water(wet) => {
                if wet >= wetness {
                    None
                } else {
                    Some(Water(wetness_here.max(1) - 1))
                }
            }
            _ => None,
        }
    }

    fn powered(&self, pow: u8) -> Option<Self> {
        // return what self becomes when it is powered
        // None if power has no effect
        match *self {
            Redstone(spow) => {
                if spow >= pow {
                    None
                } else {
                    Some(Redstone(pow.max(1) - 1))
                }
            }
            GameOfLife(false) => Some(GameOfLife(true)),
            _ => None,
        }
    }

    fn inflict(&self, other: Self, direction: Direction) -> Option<Self> {
        match *self {
            Water(0) => None,
            Water(wetness) => other.wet(wetness),
            Slug(slug_direction) => {
                if slug_direction == direction {
                    Some(Slug(slug_direction))
                } else {
                    None
                }
            }
            Redstone(0) => None,
            Redstone(powa) => other.powered(powa),
            RedstoneBlock() => other.powered(16),
            _ => None,
        }
    }

    fn resolve_infliction(&self, other: Self) -> Self {
        match (*self, other) {
            (Slug(_), Slug(_)) => Slime(),
            (Slug(direction), _) => Slug(direction),
            (_, Slug(direction)) => Slug(direction),
            (Water(weta), Water(wetb)) => Water(weta.max(wetb)),
            (Water(wet), a) => {
                if a.is_succeptible_to_liquid() {
                    Water(wet)
                } else {
                    a
                }
            }
            (a, Water(wet)) => {
                if a.is_succeptible_to_liquid() {
                    Water(wet)
                } else {
                    a
                }
            }
            (Redstone(powa), Redstone(powb)) => Redstone(powa.max(powb)),
            a => {
                println!("{:?}", a);
                unreachable!();
            }
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
    fn infliction_requested(&self) -> Option<Automata> {
        let middle = self.middle;
        [
            self.topmiddle.inflict(middle, Down),
            self.left.inflict(middle, Right),
            self.right.inflict(middle, Left),
            self.bottommiddle.inflict(middle, Up),
        ]
        .iter()
        .fold(None, |a, &b| match (a, b) {
            (Some(l), Some(r)) => Some(l.resolve_infliction(r)),
            _ => a.or(b),
        })
    }
}

pub fn next_middle(surroundings: Surroundings) -> Automata {
    if let Some(next) = surroundings.infliction_requested() {
        return next;
    }

    match surroundings.middle {
        Water(0) => Air(),
        Water(wetness) => Water(wetness.max(1) - 1),
        Redstone(pow) => Redstone(pow.max(1) - 1),
        Slug(_) => Slime(),
        a => a,
    }
}
