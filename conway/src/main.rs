#![no_main]

use conway::Universe;

entrypoint::entrypoint!(main);

pub fn main() {
    let mut universe = Universe::new();
    for i in 0..10 {
        universe.tick();
    }
    for i in 0..10 {
        for j in 0..10 {
            let idx = universe.get_index(i, j);
            let cell = universe.get_cells()[idx] as u8;
            entrypoint::io::println(cell.to_string().as_str())
        }
    }
}
