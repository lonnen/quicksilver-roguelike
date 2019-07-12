use quicksilver::{
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    prelude::*,
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
        window.clear(Color::WHITE)?;

        self.title.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;

        self.mononoki_font_info.execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate((2, window.screen_size().y as i32 - 60)),
                Img(&image),
            );
            Ok(())
        })?;

        Ok(())
    }

}

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    pos: Vector,
    glyph: char,
    color: Color
}

fn generate_map(size: Vector) -> Vec<Tile> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let mut tile = Tile {
                pos: Vector::new(x as f32, y as f32),
                glyph: '.',
                color: Color::BLACK,
            };

            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                tile.glyph = '#';
            };
            map.push(tile);
        }
    }
    map
}

#derive(Clone, Debug, PartialEq)
struct Entity {
    pos:  Vector,
    glyph: char,
    color: Color,
    hp: i32,
    max_hp:
}

fn main() {
    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("Quicksilver Roguelike", Vector::new(800, 600), settings);
}
