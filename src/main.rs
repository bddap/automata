mod automata_field;
use automata_field::AutomataField;

mod graphics;

mod automata;

use std::{thread, time};

fn main() {
    let (width, height) = graphics::size_available();

    let mut automata_field = AutomataField::new(width as usize, height as usize);
    automata_field.generate();

    let mut display = graphics::Display::new(width as usize, height as usize);

    loop {
        display.display(&automata_field);
        thread::sleep(time::Duration::from_millis(500));
        automata_field.tick();
    }
}
