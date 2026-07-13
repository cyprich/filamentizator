use core::{panic, result};

use embassy_time::Timer;
use heapless::Vec;
use log::error;

use crate::{
    Error, MAX_FILAMENT_COUNT,
    button::ButtonEvent,
    client::ApiClient,
    display::Display,
    models::Filament,
    ui::{Screen, UI},
};

enum SetFilamentPage {
    Current,  // only for updating
    Next,     // update and switch to next page if possible
    Previous, // update and switch to previous page if possible
}

pub struct Navigator<'a> {
    ui: UI<'a>,
    display: Display<'a>,
    api_client: ApiClient<'a>,
    screen: Screen<'a>,
    filaments: Vec<Filament, MAX_FILAMENT_COUNT>,
    filaments_count: i32,
    current_page: i32,
    max_page: i32,
}

impl Navigator<'_> {
    // TODO: since this is called "navigator", it should only navigate (?)
    // maybe move the getting filaments and stuff into main?
    // or choose better name for this?
    pub async fn new(
        mut ui: UI<'static>,
        mut display: Display<'static>,
        api_client: ApiClient<'static>,
    ) -> Self {
        // get filaments count
        let result = api_client.get_filaments_count().await;
        let filaments_count = match result {
            Ok(val) => val,
            Err(e) => {
                error!("{}", e);

                let screen = Screen::Error(&e);
                ui.screen = &screen;
                ui.render(&mut display).await;

                panic!();
            }
        };
        let filaments_count = self.handle_api_error(result).await;

        // set these variables - for pagination
        let current_page = 1;
        let mut max_page = filaments_count / MAX_FILAMENT_COUNT as i32;
        if filaments_count % MAX_FILAMENT_COUNT as i32 != 0 {
            max_page += 1;
        }

        let filaments = Vec::new();
        let screen = Screen::NavigationHelp;

        ui.switch_screen(&Screen::NavigationHelp);
        ui.render(&mut display).await;

        Self {
            ui,
            display,
            api_client,
            screen,
            filaments,
            filaments_count,
            current_page,
            max_page,
        }
    }

    pub async fn handle_event(&mut self, event: ButtonEvent) {
        let something_changed = match &self.ui.screen {
            Screen::Filaments(vec_inner, _, _) => false,
            Screen::NavigationHelp => {
                if matches!(event, ButtonEvent::Right) {
                    self.update_filaments(SetFilamentPage::Current).await;
                    self.screen =
                        Screen::Filaments(&self.filaments, self.current_page, self.max_page);
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        if !something_changed {
            return;
        }

        self.ui.switch_screen(&self.screen);
        self.ui.render(&mut self.display);
    }

    async fn update_filaments(&mut self, page: SetFilamentPage) {
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
        self.filaments = match result {
            Ok(val) => val,
            Err(e) => {
                error!("{}", e);

                self.screen = Screen::Error(&e);
                self.ui.switch_screen(&self.screen);
                self.ui.render(&mut self.display).await;

                panic!();
            }
        };
    }
}
