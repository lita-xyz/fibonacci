#![no_main]

// TODO: rename crate
use fibonacci::Universe;

entrypoint::entrypoint!(main);

pub fn main() {
    let mut universe = Universe::new();
    for i in 0..100 {
        universe.tick();
    }
    for i in 0..10 {
        for j in 0..10 {
            entrypoint::io::println(universe.get_index(i, j).to_string().as_str())
        }
    }
}
