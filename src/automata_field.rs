use automata::{next_middle, Automata, Direction, Surroundings};
use std::mem;
use std::slice::Chunks;
use std::vec::Vec;

pub struct AutomataField {
    width: usize,
    height: usize,
    field: Vec<Automata>,
    field_alternate: Vec<Automata>,
}

use self::Automata::*;
use self::Direction::*;

impl AutomataField {
    pub fn new(width: usize, height: usize) -> AutomataField {
        let count = width * height;
        let field: Vec<Automata> = (0..count).map(|_| Automata::Redstone(0)).collect();
        AutomataField {
            width,
            height,
            field,
            field_alternate: (0..count).map(|_| Automata::Air()).collect(),
        }
    }

    pub fn generate(&mut self) {
        let h = self.height / 2;
        for x in 0..self.width / 8 {
            self.place(RedstoneBlock(), x * 8, h);
        }

        let h = self.height / 16 * 7;
        let w = self.width / 2;
        self.place(Water(15), w, h);

        let w = self.width / 64 * 17;
        for y in (self.height / 16 * 7)..(self.height / 16 * 9) {
            self.place(Air(), w, y);
        }

        let h = self.height / 2;
        self.place(Slug(Right), 0, h);
    }

    fn place(&mut self, automata: Automata, x: usize, y: usize) {
        assert!(x < self.width && y < self.height);
        self.field[y * self.width + x] = automata;
    }

    fn automata_at(&self, x: isize, y: isize) -> Automata {
        if 0 <= x && (x as usize) < self.width && 0 <= y && (y as usize) < self.height {
            self.field[y as usize * self.width + x as usize]
        } else {
            Automata::Air()
        }
    }

    fn surroundings_for(&self, x: usize, y: usize) -> Surroundings {
        let (x, y) = (x as isize, y as isize);
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

    pub fn tick(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.field_alternate[y as usize * self.width as usize + x as usize] =
                    next_middle(self.surroundings_for(x, y))
            }
        }
        mem::swap(&mut self.field, &mut self.field_alternate);
    }

    pub fn iter(&self) -> Chunks<Automata> {
        self.field.chunks(self.width)
    }
}
