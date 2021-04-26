extern crate terminal_graphics;
extern crate terminal_size;
use self::terminal_graphics::Colour;
use self::terminal_size::{terminal_size, Height, Width};
use automata::Automata;
use automata_field::AutomataField;

pub struct Display {
    screen: terminal_graphics::Display,
    width: usize,
    height: usize,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        // Each character is two pixels tall, so if height is odd, we add an extra line.
        let text_height = height / 2 + height % 2;
        let mut screen = terminal_graphics::Display::new(width as u32, text_height as u32);
        let color = Colour::from_rgb(0, 0, 0);
        for y in 0..text_height {
            for x in 0..width {
                screen.set_pixel(x as isize, y as isize, '▄', color, color);
            }
        }
        Self {
            screen,
            width,
            height,
        }
    }

    pub fn display(&mut self, automata_field: &AutomataField) {
        for (y, row) in automata_field.iter().enumerate() {
            for (x, automata) in row.iter().enumerate() {
                assert!(y < self.height);
                assert!(x < self.width);
                let colour = color_of(*automata);
                let (x, y) = (x as isize, y as isize);
                match y % 2 {
                    0 => self.screen.get_mut_pixel(x, y / 2).set_background(colour),
                    _ => self.screen.get_mut_pixel(x, (y - 1) / 2).set_colour(colour),
                }
            }
        }
        self.screen.print();
    }
}

pub fn size_available() -> (usize, usize) {
    terminal_size()
        .map(|(Width(w), Height(h))| (w as usize, h as usize * 2))
        .unwrap_or((80, 80))
}

fn color_of(automata: Automata) -> Colour {
    use self::Automata::*;
    use self::Colour::*;
    match automata {
        Redstone(power) => {
            if power == 0 {
                BrightBlack
            } else if power < 9 {
                Red
            } else {
                BrightRed
            }
        }
        RedstoneBlock() => BrightWhite,
        GameOfLife(false) => BrightGreen,
        GameOfLife(true) => Green,
        Water(_) => Blue,
        Air() => White,
        Slug(_) => BrightYellow,
        Slime() => Yellow,
    }
}
