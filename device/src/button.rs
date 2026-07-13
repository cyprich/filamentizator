use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Timer;
use esp_hal::gpio::Input;

pub static BUTTON_EVENTS: Channel<CriticalSectionRawMutex, ButtonEvent, 8> = Channel::new();

#[derive(Debug, Clone)]
pub enum ButtonEvent {
    Up,
    Down,
    Left,
    Right,
}

#[embassy_executor::task(pool_size = 4)]
pub async fn button_task(mut button: Input<'static>, event: ButtonEvent) {
    loop {
        button.wait_for_falling_edge().await;
        BUTTON_EVENTS.send(event.clone()).await;
        Timer::after_millis(100).await; // debounce
    }
}
