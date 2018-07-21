extern crate terminal_graphics;
extern crate terminal_size;
use automata::Automata;
use automata_field::AutomataField;

use self::terminal_graphics::{Colour, Display};
use self::terminal_size::{terminal_size, Height, Width};

pub fn display(automata_field: &AutomataField) {
    let (width, height) = size_available();
    let mut screen = Display::new(width, height);
    screen.clear();
    let chunks = automata_field.iter();
    for (y, row) in chunks.enumerate() {
        for (x, automata) in row.iter().enumerate() {
            let colour = color_of(*automata);
            let (x, y) = (x as isize, y as isize);
            match y % 2 {
                0 => screen.set_pixel(x, y / 2, '▄', colour, colour),
                _ => screen.get_mut_pixel(x, (y - 1) / 2).set_colour(colour),
            }
        }
    }
    screen.print();
}

pub fn size_available() -> (u32, u32) {
    terminal_size()
        .map(|(Width(w), Height(h))| (w as u32, h as u32))
        .unwrap_or((80, 80))
}

fn color_of(automata: Automata) -> Colour {
    use self::Automata::*;
    match automata {
        Redstone(power) => Colour::from_rgb(power * 16, 0, 0),
        RedstoneBlock() => Colour::from_rgb(255, 0, 0),
        GameOfLife(false) => Colour::from_rgb(0, 255, 0),
        GameOfLife(true) => Colour::from_rgb(200, 100, 0),
        _ => Colour::from_rgb(0x66, 0x66, 0x66),
    }
}
