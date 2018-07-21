mod automata_field;
use automata_field::AutomataField;

use std::{thread, time};

fn main() {
    let (width, height) = graphics::size_available();
    let mut automata_field = AutomataField::new(width, height * 2);
    automata_field.generate();

    loop {
        graphics::display(&automata_field);
        thread::sleep(time::Duration::from_millis(50));
        automata_field.tick();
    }
}

mod graphics {
    extern crate terminal_graphics;
    extern crate terminal_size;
    use automata_field::automata::Automata;
    use automata_field::AutomataField;

    use self::terminal_graphics::{Colour, Display};
    use self::terminal_size::{terminal_size, Height, Width};

    pub fn display(automata_field: &AutomataField) {
        let (width, height) = size_available();
        let mut screen = Display::new(width, height);
        screen.clear();

        for x in 0..width {
            for y in 0..(height * 2) {
                let automata = automata_field.automata_at(x as i32, y as i32);
                let colour = color_of(automata);
                match y % 2 {
                    0 => screen.set_pixel(x as isize, y as isize / 2, '▄', colour, colour),
                    1 => screen
                        .get_mut_pixel(x as isize, (y as isize - 1) / 2)
                        .set_colour(colour),
                    _ => println!("That shouldn't happen"),
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
            GameOfLife(true) => Colour::from_rgb(0, 255, 0),
            GameOfLife(true) => Colour::from_rgb(200, 100, 0),
            _ => Colour::from_rgb(0x66, 0x66, 0x66),
        }
    }
}
