extern crate rand;

#[derive(Copy, Clone)]
pub enum Automata {
    Redstone(u8),
    RedstoneBlock(),
    GameOfLife(bool),
    Air(),
}

impl Automata {
    fn redstone_power(&self) -> u8 {
        match *self {
            Redstone(power) => power,
            RedstoneBlock() => 16,
            _ => 0,
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

pub fn next_middle(surroundings: Surroundings) -> Automata {
    match surroundings {
        Surroundings {
            middle: Redstone(power),
            topmiddle,
            left,
            right,
            bottommiddle,
            ..
        } => Redstone(
            ([topmiddle, left, right, bottommiddle]
                .iter()
                .map(Automata::redstone_power)
                .fold(0, u8::max) + rand::random::<u8>() % 4)
                .max(4) - 4,
        ),
        Surroundings {
            middle: GameOfLife(false),
            topmiddle,
            left,
            right,
            bottommiddle,
            ..
        } => GameOfLife({
            [topmiddle, left, right, bottommiddle]
                .iter()
                .map(Automata::redstone_power)
                .fold(0, u8::max) > 0
        }),
        Surroundings {
            middle: GameOfLife(true),
            topmiddle,
            left,
            right,
            bottommiddle,
            ..
        } => RedstoneBlock(),
        Surroundings { middle, .. } => middle,
    }
}
