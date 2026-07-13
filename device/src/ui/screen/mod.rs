use embedded_graphics::{
    Drawable,
    image::{Image, ImageRaw},
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_5X7, FONT_6X12, FONT_8X13},
    },
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use heapless::{String, Vec};

use crate::{Error, MAX_FILAMENT_COUNT, MAX_STRING_LENGTH, models::Filament};
use crate::{display::Display, trunc_str};

use core::fmt::{Pointer, Write};
use log::info;

mod draw;
use draw::*;

#[derive(Debug)]
pub enum Screen<'a> {
    Welcome(Option<&'a str>),
    Filaments(Vec<Filament, MAX_FILAMENT_COUNT>, i32, i32),
    NavigationHelp,
    Error(&'a Error),
}

impl Screen<'_> {
    pub async fn draw(&self, display: &mut Display<'_>) {
        match self {
            Screen::Welcome(message) => draw_welcome(display, &message.as_deref()).await,
            Screen::Filaments(f, current_page, max_page) => {
                draw_filaments(display, f, *current_page, *max_page).await
            }
            Self::NavigationHelp => draw_navigation_help(display).await,
            Screen::Error(e) => draw_error(display, e).await,
        }
    }

    pub async fn render(&self, display: &mut Display<'_>) {
        display.clear().await;
        self.draw(display).await;
        display.flush().await;
    }
}
