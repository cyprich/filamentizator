use heapless::Vec;

use crate::display::Display;
use crate::{Error, MAX_FILAMENT_COUNT, models::Filament};

mod draw;
use draw::*;

#[derive(Debug)]
pub enum Screen<'a> {
    Welcome,
    // Filaments(Vec<Filament, MAX_FILAMENT_COUNT>, i32, i32),
    Filaments {
        filaments: Vec<Filament, MAX_FILAMENT_COUNT>,
        current_page: i32,
        max_page: i32,
        seleted_filament: i32,
    },
    Filament(Filament),
    NavigationHelp,
    Info(&'a str),
    Error(&'a Error),
}

impl Screen<'_> {
    pub async fn draw(&self, display: &mut Display<'_>) {
        match self {
            Screen::Welcome => draw_welcome(display).await,
            Screen::Filaments {
                filaments,
                current_page,
                max_page,
                seleted_filament,
            } => {
                draw_filaments(
                    display,
                    filaments,
                    *current_page,
                    *max_page,
                    *seleted_filament,
                )
                .await
            }
            Self::Filament(filament) => draw_filament(display, filament).await,
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
