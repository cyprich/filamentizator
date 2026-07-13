use core::result;

use embassy_time::Timer;
use heapless::Vec;

use crate::{
    MAX_FILAMENT_COUNT,
    button::ButtonEvent,
    client::ApiClient,
    display::Display,
    handle_get_filaments_error,
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
        let filaments_count = handle_get_filaments_error(result, &mut ui, &mut display).await;

        // set these variables - for pagination
        let current_page = 1;
        let mut max_page = filaments_count / MAX_FILAMENT_COUNT as i32;
        if filaments_count % MAX_FILAMENT_COUNT as i32 != 0 {
            max_page += 1;
        }

        let filaments = Vec::new();

        ui.switch_screen(&Screen::NavigationHelp);
        ui.render(&mut display).await;

        Self {
            ui,
            display,
            api_client,
            filaments,
            filaments_count,
            current_page,
            max_page,
        }
    }

    pub async fn handle_event(&'a mut self, event: ButtonEvent) {
        let new_screen = match &self.ui.screen {
            Screen::Filaments(vec_inner, _, _) => None,
            Screen::NavigationHelp => {
                if matches!(event, ButtonEvent::Right) {
                    self.update_filaments(SetFilamentPage::Current).await;
                    let filaments = &self.filaments;
                    let screen = Screen::Filaments(&filaments, self.current_page, self.max_page);
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

        let new_screen = new_screen.unwrap();
        self.ui.switch_screen(&new_screen);
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
        self.filaments = handle_get_filaments_error(result, &mut self.ui, &mut self.display).await;
    }
}
