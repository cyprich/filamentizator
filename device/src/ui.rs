use embedded_graphics::{
    Drawable,
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_4X6, FONT_5X7, FONT_6X12, FONT_8X13},
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use heapless::Vec;
use log::info;

use crate::{MAX_FILAMENT_COUNT, models::Filament};
use crate::{display::Display, models::FilamentSimple};

// padding
const PAD: i32 = 4;

#[derive(Debug)]
pub enum Screen {
    Welcome,
    Filaments(Vec<Filament, MAX_FILAMENT_COUNT>),
}

impl Screen {
    pub async fn draw(&self, display: &mut Display<'_>) {
        match self {
            Screen::Welcome => draw_welcome(display).await,
            Screen::Filaments(f) => draw_filaments(display, f).await,
        }
    }
}

pub enum Font {
    Heading,
    Text,
    Description,
}

impl Font {
    pub fn get(&self) -> MonoTextStyle<'_, BinaryColor> {
        match self {
            Font::Heading => MonoTextStyleBuilder::new()
                .font(&FONT_8X13)
                .text_color(BinaryColor::On)
                .build(),
            Font::Text => MonoTextStyleBuilder::new()
                .font(&FONT_6X12)
                .text_color(BinaryColor::On)
                .build(),
            Font::Description => MonoTextStyleBuilder::new()
                .font(&FONT_5X7)
                .text_color(BinaryColor::On)
                .build(),
        }
    }

    pub fn height(&self) -> i32 {
        match self {
            Font::Heading => 8,
            Font::Text => 6,
            Font::Description => 5,
        }
    }
}

#[derive(Debug)]
pub struct UI {
    pub screen: Screen,
}

impl UI {
    pub fn new() -> Self {
        Self {
            screen: Screen::Welcome,
        }
    }

    pub async fn draw(&self, display: &mut Display<'_>) {
        self.screen.draw(display).await;
    }

    pub fn switch_screen(&mut self, screen: Screen) {
        self.screen = screen;
    }
}

async fn draw_welcome(display: &mut Display<'_>) {
    let x = PAD;
    let mut y = PAD;

    let welcome = Text::with_baseline(
        "Welcome to",
        Point::new(x, y),
        Font::Text.get(),
        Baseline::Top,
    );

    y += Font::Text.height() + PAD * 2;

    let fila = Text::with_baseline(
        "Filamentizator",
        Point::new(x, y),
        Font::Heading.get(),
        Baseline::Top,
    );

    y += Font::Heading.height() + PAD * 2;

    let loading = Text::with_baseline(
        "Loading...",
        Point::new(x, y),
        Font::Description.get(),
        Baseline::Top,
    );

    welcome.draw(display).unwrap();
    fila.draw(display).unwrap();
    loading.draw(display).unwrap();
}

async fn draw_filaments(display: &mut Display<'_>, filaments: &Vec<Filament, MAX_FILAMENT_COUNT>) {
    let x = PAD;
    let mut y = PAD;

    let text = Text::with_baseline(
        "Filaments",
        Point::new(x, y),
        Font::Description.get(),
        Baseline::Top,
    );

    text.draw(display).unwrap();
}
