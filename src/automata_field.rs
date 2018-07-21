extern crate rand;

use automata::{next_middle, Automata, Surroundings};
use std::mem;
use std::slice::Chunks;
use std::vec::Vec;

pub struct AutomataField {
    width: u32,
    height: u32,
    pub field: Vec<Automata>,
    field_alternate: Vec<Automata>,
}

impl AutomataField {
    pub fn new(width: u32, height: u32) -> AutomataField {
        let field: Vec<Automata> = (0..width * height).map(|_| Automata::Redstone(0)).collect();
        AutomataField {
            width,
            height,
            field,
            field_alternate: (0..width * height).map(|_| Automata::Air()).collect(),
        }
    }

    pub fn generate(&mut self) {
        self.spread(Automata::RedstoneBlock(), 4);
        self.spread(Automata::GameOfLife(false), 16);
    }

    fn spread(&mut self, automata: Automata, count: u32) {
        for _ in 0..count {
            let x = rand::random::<u32>() % self.width;
            let y = rand::random::<u32>() % self.height;
            self.place(automata, x, y);
        }
    }

    fn place(&mut self, automata: Automata, x: u32, y: u32) {
        assert!(x < self.width && y < self.height);
        self.field[(y as u32 * self.width + x as u32) as usize] = automata;
    }

    fn automata_at(&self, x: i32, y: i32) -> Automata {
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
        self.field.chunks(self.width as usize)
    }
}

// struct AutomataFieldIterator {
//     progress: usize,
//     width: usize,
//     height: usize,
//     content: Vec<Automata>,
// }

// impl Iterator for AutomataFieldIterator {
//     type Item = (usize, usize, Automata);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.progress < self.height * self.width {
//             let (x, y, p) = (
//                 self.progress % self.width,
//                 self.progress / self.width,
//                 self.progress,
//             );
//             assert!(x + y * self.width == p);
//             self.progress = self.progress + 1;
//             Some((x, y, self.content[p]))
//         } else {
//             None
//         }
//     }
// }

// struct AutomataFieldIterator {
//     content: Chunks<Automata>,
//     currentslice: Slice<Automata>,
// }
