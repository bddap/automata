mod automata_field;
use automata_field::AutomataField;

mod graphics;

mod automata;

use std::{thread, time};

fn main() {
    let (width, height) = graphics::size_available();
    let mut automata_field = AutomataField::new(width, height * 2);
    automata_field.generate();

    loop {
        graphics::display(&automata_field);
        thread::sleep(time::Duration::from_millis(500));
        automata_field.tick();
    }
}
