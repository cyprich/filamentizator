use esp_hal::gpio::OutputPin;

pub struct RGBLed<R, G, B> {
    // actual pins
    r: R,
    g: G,
    b: B,
    // value 0-255
    value: (u8, u8, u8),
}

impl<R, G, B> RGBLed<R, G, B>
where
    R: OutputPin,
    G: OutputPin,
    B: OutputPin,
{
    pub fn new(r: R, g: G, b: B) -> Self {
        Self {
            r,
            g,
            b,
            value: (0, 0, 0),
        }
    }

    pub fn rgb(&mut self, r: u8, g: u8, b: u8) {
        self.value = (r, g, b)
    }

    pub fn hex(&mut self, hex: &str) {
        // TODO: hex to rgb
        self.value = (0, 0, 0);
    }

    fn shine(&self) {}
}
