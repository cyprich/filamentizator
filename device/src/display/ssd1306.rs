use embedded_graphics::{Pixel, pixelcolor::BinaryColor, prelude::*};
use esp_hal::{
    Async,
    gpio::{InputPin, OutputPin},
    i2c::master::{Config as I2cConfig, I2c},
    time::Rate,
};
use ssd1306::{
    I2CDisplayInterface, Ssd1306Async,
    mode::{BufferedGraphicsModeAsync, DisplayConfigAsync},
    prelude::*,
    size::DisplaySize128x64,
};

// TODO: user-configurable display size
pub struct Display<'a> {
    inner: ssd1306::Ssd1306Async<
        I2CInterface<I2c<'a, Async>>,
        DisplaySize128x64,
        BufferedGraphicsModeAsync<DisplaySize128x64>,
    >,
    // TODO: display size
    framebuffer: [u8; 128 * 64],
}

impl<'a> Display<'a> {
    pub fn new(
        i2c: esp_hal::peripherals::I2C0<'a>,
        scl: impl InputPin + OutputPin + 'a,
        sda: impl InputPin + OutputPin + 'a,
    ) -> Self {
        let i2c_config = I2cConfig::default().with_frequency(Rate::from_khz(100));

        let i2c_bus = I2c::new(i2c, i2c_config)
            .unwrap()
            .with_scl(scl)
            .with_sda(sda)
            .into_async();

        let i2c_interface = I2CDisplayInterface::new(i2c_bus);

        let mut display =
            Ssd1306Async::new(i2c_interface, DisplaySize128x64, DisplayRotation::Rotate0)
                .into_buffered_graphics_mode();

        let framebuffer = [0u8; 128 * 64];

        Self {
            inner: display,
            framebuffer,
        }
    }

    // TODO: this needs to be done before using
    // compilation time check
    pub async fn init(&mut self) {
        self.inner.init().await.unwrap();
    }

    pub async fn flush(&mut self) {
        self.inner.flush().await.unwrap();
    }

    pub async fn clear(&mut self) {
        self.inner.clear(BinaryColor::Off).unwrap();
    }
}

///////////////////////////////////////////////////////

// implementations from embedded_graphics

impl DrawTarget for Display<'_> {
    type Color = BinaryColor;

    type Error = crate::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.inner
            .draw_iter(pixels)
            .map_err(|_| crate::Error::SsdDisplayError)
    }

    // fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    // where
    //     I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    // {
    //     for Pixel(coord, color) in pixels.into_iter() {
    //         let x = coord.x;
    //         let y = coord.y;
    //
    //         if x >= 0 && x < 128 && y >= 0 && y < 64 {
    //             let index = (y * 128 + x) as usize;
    //             self.framebuffer[index] = color.is_on() as u8;
    //         }
    //     }
    //
    //     Ok(())
    // }
}

impl OriginDimensions for Display<'_> {
    fn size(&self) -> Size {
        Size::new(128, 64)
    }
}
