use esp_hal::gpio::{DriveMode::PushPull, OutputPin};

use esp_hal::ledc::{
    self, Ledc, LowSpeed,
    channel::{self, Channel, ChannelIFace, config::Config},
    timer::{self, TimerIFace},
};

use log::error;

use crate::MAX_LED_BRIGHTNESS;

pub struct RGBLed<'a> {
    r: Channel<'a, LowSpeed>,
    g: Channel<'a, LowSpeed>,
    b: Channel<'a, LowSpeed>,
}

impl<'a> RGBLed<'a> {
    pub fn new<R, G, B>(
        r: R,
        g: G,
        b: B,
        mut ledc: Ledc<'a>,
        timer: &'a mut timer::Timer<'a, LowSpeed>,
    ) -> Self
    where
        R: OutputPin + 'a,
        G: OutputPin + 'a,
        B: OutputPin + 'a,
    {
        ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk);

        let _ = timer.configure(timer::config::Config {
            duty: timer::config::Duty::Duty8Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: esp_hal::time::Rate::from_khz(24),
        });

        let mut r = ledc.channel(channel::Number::Channel0, r);
        let mut g = ledc.channel(channel::Number::Channel1, g);
        let mut b = ledc.channel(channel::Number::Channel2, b);

        let conf = Config {
            timer,
            duty_pct: 0,
            drive_mode: PushPull,
        };

        r.configure(conf).unwrap();
        g.configure(conf).unwrap();
        b.configure(conf).unwrap();

        Self { r, g, b }
    }

    pub fn rgb(&mut self, r: u8, g: u8, b: u8) {
        fn percent(val: u8) -> u8 {
            (val as u16 * MAX_LED_BRIGHTNESS / 255) as u8
        }

        let r = percent(r);
        let g = percent(g);
        let b = percent(b);

        let _ = self.r.set_duty(r);
        let _ = self.g.set_duty(g);
        let _ = self.b.set_duty(b);
    }

    pub fn hex(&mut self, hex: &str) {
        if hex.len() != 6 {
            error!("Invalid HEX value: {}", hex);
            return;
        }

        let val = match u32::from_str_radix(hex, 16) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to parse HEX: {}", e);
                return;
            }
        };

        let r = ((val >> 16) & 0xff) as u8;
        let g = ((val >> 8) & 0xff) as u8;
        let b = (val & 0xff) as u8;

        self.rgb(r, g, b);
    }

    pub async fn strobe_rgb(&mut self, r: u8, g: u8, b: u8, millis: Option<u64>) {
        let duration = embassy_time::Duration::from_millis(millis.unwrap_or(50));

        for _ in 0..3 {
            self.rgb(r, g, b);
            embassy_time::Timer::after(duration).await;
            self.rgb(0, 0, 0);
            embassy_time::Timer::after(duration).await;
        }
    }

    // TODO: strobe_hex, but will i need it?
}
