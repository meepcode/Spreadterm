use spreadterm::{interface::Interface, grid::TextGrid};
use ncurses::*;

fn main() {
    let mut interface = Interface::new((10, 10));
    let mut grid = TextGrid::new((10, 10));
    interface.setup();
    loop {
        if !interface.update(&mut grid) {
            break;
        }
    }
    endwin();
}


