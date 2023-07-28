use std::{
    fs::{self, create_dir, remove_dir_all, File},
    io::Read,
};

use font_kit::{font::Font, loader::Loader};
use pathfinder_geometry::vector::vec2f;
use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};
use ttf_parser::Face;
use ttf_word_wrap::{TTFParserMeasure, WhiteSpaceWordWrap, Wrap};

enum Colour {
    White,
    Red,
}

const PRIMARY_FONT_SIZE: f32 = 90.;
const FOOTER_FONT_SIZE: f32 = 20.;
const MARGIN: f32 = 32.;
const CARD_SIZE: (i32, i32) = (400, 600);

fn main() {
    let _ = remove_dir_all("cards");
    create_dir("cards").unwrap();

    let mut draw_target = DrawTarget::new(CARD_SIZE.0, CARD_SIZE.1);
    let mut primary_font = File::open("src/fonts/LeagueSpartan.ttf").unwrap();
    let mut footer_font = File::open("src/fonts/Sanchez.ttf").unwrap();
    let white = SolidSource::from_unpremultiplied_argb(255, 255, 255, 255);
    let red = SolidSource::from_unpremultiplied_argb(255, 255, 71, 71);
    let font_bytes = {
        let mut font_buffer = Vec::new();
        primary_font.read_to_end(&mut font_buffer).unwrap();
        font_buffer
    };
    let font_face = Face::from_slice(font_bytes.as_slice(), 0).unwrap();
    let measure = TTFParserMeasure::new(&font_face);
    let word_wrap = &WhiteSpaceWordWrap::new(7000, &measure);
    let (white_cards, red_cards, footer_text) = {
        let raw = fs::read_to_string("text").unwrap();
        assert_eq!(raw.split("\n\n").count(), 3);
        [0, 1, 2]
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

    white_cards
        .iter()
        .map(|card| (card, Colour::White))
        .chain(red_cards.iter().map(|card| (card, Colour::Red)))
        .for_each(|card| {
            draw_target.fill_rect(
                0.,
                0.,
                CARD_SIZE.0 as f32,
                CARD_SIZE.1 as f32,
                &Source::Solid(match card.1 {
                    Colour::White => white,
                    Colour::Red => red,
                }),
                &DrawOptions::new(),
            );
            card.0
                .as_str()
                .wrap(word_wrap)
                .enumerate()
                .for_each(|(index, line)| {
                    write_text(
                        &mut draw_target,
                        &Loader::from_file(&mut primary_font, 0).unwrap(),
                        PRIMARY_FONT_SIZE,
                        line,
                        Point::new(
                            MARGIN,
                            PRIMARY_FONT_SIZE + MARGIN + index as f32 * PRIMARY_FONT_SIZE,
                        ),
                        &Source::Solid(match card.1 {
                            Colour::White => red,
                            Colour::Red => white,
                        }),
                        &DrawOptions::new(),
                        1.1,
                    );
                });

            assert_eq!(footer_text.len(), 1);
            write_text(
                &mut draw_target,
                &Loader::from_file(&mut footer_font, 0).unwrap(),
                FOOTER_FONT_SIZE,
                footer_text.first().unwrap(),
                Point::new(MARGIN, CARD_SIZE.1 as f32 - MARGIN),
                &Source::Solid(match card.1 {
                    Colour::White => SolidSource::from_unpremultiplied_argb(255, 70, 70, 70),
                    Colour::Red => white,
                }),
                &DrawOptions::new(),
                2.,
            );
            draw_target
                .write_png(format!("cards/{}.png", card.0))
                .unwrap();
        });
}

fn write_text(
    draw_target: &mut DrawTarget,
    font: &Font,
    point_size: f32,
    text: &str,
    start: Point,
    src: &Source,
    options: &DrawOptions,
    character_space_multiplier: f32,
) {
    {
        let mut start = vec2f(start.x, start.y);
        let mut ids = Vec::new();
        let mut positions = Vec::new();
        for c in text.chars() {
            let id = font.glyph_for_char(c).unwrap();
            ids.push(id);
            positions.push(Point::new(start.x(), start.y()));
            start +=
                font.advance(id).unwrap() * point_size / 24. / 96. * character_space_multiplier;
        }
        draw_target.draw_glyphs(font, point_size, &ids, &positions, src, options);
    }
}
