use core::panic;

use heapless::Vec;
use log::error;

use crate::{
    MAX_FILAMENT_COUNT, api_client::ApiClient, button::ButtonEvent, display::Display,
    models::Filament, ui::Screen,
};

enum SetFilamentPage {
    Current,  // only for updating
    Next,     // update and switch to next page if possible
    Previous, // update and switch to previous page if possible
}

pub struct Navigator<'a> {
    // structs
    display: Display<'a>,
    api_client: ApiClient<'a>,
    screen: Screen<'a>,

    // pagination stuff
    current_page: i32,
    max_page: i32,

    // quitting program
    should_exit: bool,
}

impl Navigator<'_> {
    // TODO: since this is called "navigator", it should only navigate (?)
    // maybe move the getting filaments and stuff into main?
    // or choose better name for this?
    pub async fn new(mut display: Display<'static>, api_client: ApiClient<'static>) -> Self {
        // get filaments count
        let result = api_client.get_filaments_count().await;
        let filaments_count = match result {
            Ok(val) => val,
            Err(e) => {
                error!("{}", e);
                Screen::Error(&e).render(&mut display).await;
                panic!();
            }
        };

        // set these variables - for pagination
        let current_page = 1;
        let mut max_page = filaments_count / MAX_FILAMENT_COUNT as i32;
        if filaments_count % MAX_FILAMENT_COUNT as i32 != 0 {
            max_page += 1;
        }

        let screen = Screen::NavigationHelp;
        screen.render(&mut display).await;

        Self {
            display,
            api_client,
            screen,
            current_page,
            max_page,
            should_exit: false,
        }
    }

    pub async fn handle_event(&mut self, event: &ButtonEvent) {
        let new_screen = match &self.screen {
            // TODO: all screens
            Screen::Filaments(_vec_inner, _, _) => None,
            Screen::NavigationHelp => {
                if matches!(event, ButtonEvent::Right) {
                    let filaments = self.update_filaments(SetFilamentPage::Current).await;
                    let screen = Screen::Filaments(filaments, self.current_page, self.max_page);
                    Some(screen)
                } else {
                    None
                }
            }
            _ => None,
        };
        if new_screen.is_none() {
            return;
        }

        self.screen = new_screen.unwrap();
        self.screen.render(&mut self.display).await;
    }

    async fn update_filaments(
        &mut self,
        page: SetFilamentPage,
    ) -> Vec<Filament, MAX_FILAMENT_COUNT> {
        let new_page = match page {
            SetFilamentPage::Current => self.current_page,
            SetFilamentPage::Next => {
                if self.current_page + 1 > self.max_page {
                    self.current_page
                } else {
                    self.current_page + 1
                }
            }
            SetFilamentPage::Previous => {
                if self.current_page - 1 < 1 {
                    self.current_page
                } else {
                    self.current_page - 1
                }
            }
        };

        let result = self.api_client.get_filaments(new_page).await;
        match result {
            Ok(val) => val,
            Err(e) => {
                error!("{}", e);
                Screen::Error(&e).render(&mut self.display).await;
                panic!();
            }
        }
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}
