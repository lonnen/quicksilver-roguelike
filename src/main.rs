use quicksilver::{
    geom::Vector,
    graphics::{Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Future,
    Result,
};

struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
}

impl State for Game {
    /// load assets and initialize
    fn new() -> Result<Self> {
        let font_mononoki = "mononoki-Regular.ttf";

        let title = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("QSR", &FontStyle::new(72.0, Color::BLACK))
        }));

        let mononoki_font_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render(
                "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
                &FontStyle::new(20.0, Color::BLACK))
        }));

        Ok(Self {
            title,
            mononoki_font_info
        })
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
