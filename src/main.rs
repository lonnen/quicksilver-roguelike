use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings, State, Window},
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
    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("Quicksilver Roguelike", Vector::new(800, 600), settings);
}
