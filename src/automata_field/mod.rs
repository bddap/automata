pub mod automata;

use self::automata::{next_middle, Automata, Surroundings};
use std::mem;
use std::slice::Iter;
use std::vec::Vec;

use std::iter::Enumerate;

pub struct AutomataField {
    width: u32,
    height: u32,
    pub field: Vec<Automata>,
    field_alternate: Vec<Automata>,
}

impl AutomataField {
    pub fn new(width: u32, height: u32) -> AutomataField {
        AutomataField {
            width,
            height,
            field: (0..width * height).map(|_| Automata::Air()).collect(),
            field_alternate: (0..width * height).map(|_| Automata::Air()).collect(),
        }
    }

    pub fn automata_at(&self, x: i32, y: i32) -> Automata {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.field[(y as u32 * self.width + x as u32) as usize]
        } else {
            Automata::Air()
        }
    }

    fn surroundings_for(&self, x: u32, y: u32) -> Surroundings {
        let (x, y) = (x as i32, y as i32);
        Surroundings {
            topleft: self.automata_at(x - 1, y - 1),
            topmiddle: self.automata_at(x, y - 1),
            topright: self.automata_at(x + 1, y - 1),
            left: self.automata_at(x - 1, y),
            middle: self.automata_at(x, y),
            right: self.automata_at(x + 1, y),
            bottomleft: self.automata_at(x - 1, y + 1),
            bottommiddle: self.automata_at(x, y + 1),
            bottomright: self.automata_at(x + 1, y + 1),
        }
    }

    fn tick(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.field_alternate[y as usize * self.width as usize + x as usize] =
                    next_middle(self.surroundings_for(x, y))
            }
        }
        mem::swap(&mut self.field, &mut self.field_alternate);
    }
}
