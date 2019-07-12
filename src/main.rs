use quicksilver::{
    lifecycle::{State, Window},
    Result,
};

struct Game;

impl State for Game {
    /// load assets and initialize
    fn new() -> Result<Self> {
        Ok(Self)
    }

    /// process human inputs, update game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

    /// output to the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

}

fn main() {
    println!("Hello, world!");
}
