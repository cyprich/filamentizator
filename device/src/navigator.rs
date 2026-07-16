use core::panic;

use heapless::Vec;
use log::{error, warn};

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
        warn!("new navigator");
        // get filaments count
        let result = api_client.get_filaments_count().await;
        warn!("new filaments");
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
            Screen::Filaments(_, _, _) => match event {
                ButtonEvent::Up => {
                    if let Some(val) = self.update_filaments(SetFilamentPage::Previous).await {
                        let screen = Screen::Filaments(val, self.current_page, self.max_page);
                        Some(screen)
                    } else {
                        None
                    }
                }
                ButtonEvent::Down => {
                    if let Some(val) = self.update_filaments(SetFilamentPage::Next).await {
                        let screen = Screen::Filaments(val, self.current_page, self.max_page);
                        Some(screen)
                    } else {
                        None
                    }
                }
                ButtonEvent::Right => {
                    // TODO: one filament screen
                    None
                }
                ButtonEvent::Left => {
                    // dont do anything? idk
                    None
                }
            },
            Screen::NavigationHelp => {
                if matches!(event, ButtonEvent::Right) {
                    if let Some(val) = self.update_filaments(SetFilamentPage::Current).await {
                        let screen = Screen::Filaments(val, self.current_page, self.max_page);
                        Some(screen)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            // dont do anything on these screens
            Screen::Welcome | Screen::Info(_) | Screen::Error(_) => None,
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
    ) -> Option<Vec<Filament, MAX_FILAMENT_COUNT>> {
        let new_page = match page {
            SetFilamentPage::Current => Some(self.current_page),
            SetFilamentPage::Next => {
                if self.current_page < self.max_page {
                    Some(self.current_page + 1)
                } else {
                    None
                }
            }
            SetFilamentPage::Previous => {
                if self.current_page > 1 {
                    Some(self.current_page - 1)
                } else {
                    None
                }
            }
        }?;

        self.current_page = new_page;

        let result = self.api_client.get_filaments(self.current_page).await;
        match result {
            Ok(val) => Some(val),
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
