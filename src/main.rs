use std::fs::{self, create_dir, remove_dir_all, File};

use font_kit::loader::Loader;
use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};

enum Colour {
    White,
    Red,
}

fn main() {
    let _ = remove_dir_all("cards");
    create_dir("cards").unwrap();
    let primary_font_size = 30.;
    let footer_font_size = 16.;
    let margin = 16.;
    let card_size = (400, 600);
    let mut draw_target = DrawTarget::new(card_size.0, card_size.1);
    let mut primary_font = File::open("src/fonts/Signika.ttf").unwrap();
    let mut footer_font = File::open("src/fonts/Montserrat.ttf").unwrap();
    let (white_cards, red_cards) = {
        let raw = fs::read_to_string("text").unwrap();
        [0, 1]
            .map(|index| {
                raw.split("\n\n")
                    .map(|part| {
                        part.split('\n')
                            .filter_map(|line| {
                                if line.is_empty() {
                                    None
                                } else {
                                    Some(line.to_string())
                                }
                            })
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>()[index]
                    .clone()
            })
            .into()
    };
    let white = SolidSource::from_unpremultiplied_argb(255, 255, 255, 255);
    let red = SolidSource::from_unpremultiplied_argb(255, 255, 71, 71);
    white_cards
        .iter()
        .map(|card| (card, Colour::White))
        .chain(red_cards.iter().map(|card| (card, Colour::Red)))
        .for_each(|card| {
            draw_target.fill_rect(
                0.,
                0.,
                card_size.0 as f32,
                card_size.1 as f32,
                &Source::Solid(match card.1 {
                    Colour::White => white,
                    Colour::Red => red,
                }),
                &DrawOptions::new(),
            );
            draw_target.draw_text(
                &Loader::from_file(&mut primary_font, 0).unwrap(),
                primary_font_size,
                card.0,
                Point::new(margin, primary_font_size + margin),
                &Source::Solid(match card.1 {
                    Colour::White => red,
                    Colour::Red => white,
                }),
                &DrawOptions::new(),
            );
            draw_target.draw_text(
                &Loader::from_file(&mut footer_font, 0).unwrap(),
                footer_font_size,
                card.0,
                Point::new(margin, card_size.0 as f32),
                &Source::Solid(match card.1 {
                    Colour::White => red,
                    Colour::Red => white,
                }),
                &DrawOptions::new(),
            );
            draw_target
                .write_png(format!("cards/{}.png", card.0))
                .unwrap();
        });
}
