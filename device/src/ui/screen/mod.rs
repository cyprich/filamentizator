use embedded_graphics::{Drawable, prelude::*};
use heapless::Vec;

use crate::display::Display;
use crate::{Error, MAX_FILAMENT_COUNT, models::Filament};

use core::fmt::Write;

mod draw;
use draw::*;

#[derive(Debug)]
pub enum Screen<'a> {
    Welcome,
    Filaments(Vec<Filament, MAX_FILAMENT_COUNT>, i32, i32),
    NavigationHelp,
    Info(&'a str),
    Error(&'a Error),
}

impl Screen<'_> {
    pub async fn draw(&self, display: &mut Display<'_>) {
        match self {
            Screen::Welcome => draw_welcome(display).await,
            Screen::Filaments(f, current_page, max_page) => {
                draw_filaments(display, f, *current_page, *max_page).await
            }
            Self::NavigationHelp => draw_navigation_help(display).await,
            Screen::Info(info) => draw_info(display, info).await,
            Screen::Error(error) => draw_error(display, error).await,
        }
    }

    pub async fn render(&self, display: &mut Display<'_>) {
        display.clear().await;
        self.draw(display).await;
        display.flush().await;
    }
}
