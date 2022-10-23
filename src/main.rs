use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.cls();
        context.print(1, 1, "Hello, terminal!");
    }
}

fn main() {
    println!("Hello, world!");
}
