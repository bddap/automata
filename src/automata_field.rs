extern crate rand;

use automata::{next_middle, Automata, Surroundings};
use std::mem;
use std::slice::Chunks;
use std::vec::Vec;

pub struct AutomataField {
    width: usize,
    height: usize,
    field: Vec<Automata>,
    field_alternate: Vec<Automata>,
}

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
        self.spread(Automata::RedstoneBlock(), 4);
        self.spread(Automata::GameOfLife(false), 16);
    }

    fn spread(&mut self, automata: Automata, count: u32) {
        for _ in 0..count {
            let x = rand::random::<usize>() % self.width;
            let y = rand::random::<usize>() % self.height;
            self.place(automata, x, y);
        }
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
