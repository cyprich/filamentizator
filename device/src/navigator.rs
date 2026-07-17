use core::{cmp::min, panic};

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

    // filaments pagination stuff
    filaments_count: i32,
    current_page: i32,
    max_page: i32,
    selected_filament: i32,

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
        let selected_filament = 1;

        let screen = Screen::NavigationHelp;
        screen.render(&mut display).await;

        Self {
            // structs
            display,
            api_client,
            screen,
            // filaments pagination
            filaments_count,
            current_page,
            selected_filament,
            max_page,
            // exit state
            should_exit: false,
        }
    }

    pub async fn handle_event(&mut self, event: &ButtonEvent) {
        let new_screen = match &self.screen {
            // TODO: all screens
            Screen::Filaments {
                filaments,
                current_page: _,
                max_page: _,
                seleted_filament: _,
            } => match event {
                // navigation in Filaments screen
                ButtonEvent::Up => {
                    // check if we are staying on this page
                    if self.selected_filament > 1 {
                        // just select previous filament
                        self.selected_filament -= 1;
                        Some(Screen::Filaments {
                            filaments: filaments.clone(),
                            current_page: self.current_page,
                            max_page: self.max_page,
                            seleted_filament: self.selected_filament,
                        })
                    } else {
                        // we are going to the next page
                        // if update was successfull/if there is next page
                        if let Some(val) = self.update_filaments(SetFilamentPage::Previous).await {
                            Some(Screen::Filaments {
                                filaments: val,
                                current_page: self.current_page,
                                max_page: self.max_page,
                                seleted_filament: self.selected_filament,
                            })
                        } else {
                            None
                        }
                    }
                }
                ButtonEvent::Down => {
                    if self.selected_filament < MAX_FILAMENT_COUNT as i32 {
                        self.selected_filament += 1;

                        // fix "invisible filament selected"; if screen is not filled entirely with filaments
                        if self.current_page == self.max_page {
                            self.selected_filament = min(
                                self.selected_filament,
                                self.filaments_count % MAX_FILAMENT_COUNT as i32,
                            );
                        }

                        Some(Screen::Filaments {
                            filaments: filaments.clone(),
                            current_page: self.current_page,
                            max_page: self.max_page,
                            seleted_filament: self.selected_filament,
                        })
                    } else {
                        if let Some(val) = self.update_filaments(SetFilamentPage::Next).await {
                            Some(Screen::Filaments {
                                filaments: val,
                                current_page: self.current_page,
                                max_page: self.max_page,
                                seleted_filament: self.selected_filament,
                            })
                        } else {
                            None
                        }
                    }
                }
                ButtonEvent::Right => Some(Screen::Filament(
                    filaments[self.selected_filament as usize - 1].clone(),
                )),
                ButtonEvent::Left => {
                    // dont do anything? go to navigation help? idk what i want
                    None
                }
            },
            Screen::NavigationHelp => {
                if matches!(event, ButtonEvent::Right) {
                    if let Some(val) = self.update_filaments(SetFilamentPage::Current).await {
                        let screen = Screen::Filaments {
                            filaments: val,
                            current_page: self.current_page,
                            max_page: self.max_page,
                            seleted_filament: self.selected_filament,
                        };
                        Some(screen)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            Screen::Filament(_) => {
                if matches!(event, ButtonEvent::Left) {
                    let result = self.update_filaments(SetFilamentPage::Current).await;
                    if let Some(val) = result {
                        Some(Screen::Filaments {
                            filaments: val,
                            current_page: self.current_page,
                            max_page: self.max_page,
                            seleted_filament: self.selected_filament,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            // TODO: dont do anything on these screens?
            Screen::Welcome | Screen::Info(_) | Screen::Error(_) => None,
        };

        if new_screen.is_none() {
            return;
        }

        // update and render screen
        self.screen = new_screen.unwrap();
        self.screen.render(&mut self.display).await;
    }

    async fn update_filaments(
        &mut self,
        page: SetFilamentPage,
    ) -> Option<Vec<Filament, MAX_FILAMENT_COUNT>> {
        match page {
            // just update
            SetFilamentPage::Current => Some(self.current_page),
            // check if we are going to the next page
            SetFilamentPage::Next => {
                if self.current_page < self.max_page {
                    self.current_page += 1;
                    self.selected_filament = 1;
                    Some(self.current_page)
                } else {
                    None
                }
            }
            // check if we are going to the previous page
            SetFilamentPage::Previous => {
                if self.current_page > 1 {
                    self.current_page -= 1;
                    self.selected_filament = MAX_FILAMENT_COUNT as i32;
                    Some(self.current_page)
                } else {
                    None
                }
            }
        }?;

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
