#![no_main]

// TODO: rename crate
use fibonacci::Universe;

entrypoint::entrypoint!(main);

pub fn main() {
    let mut universe = Universe::new();
    for i in 0..100 {
        universe.tick();
    }

//     entrypoint::io::println(&n.to_string());
//     entrypoint::io::println("-th fibonacci number is:");
//     entrypoint::io::println(&b.to_string());
}
