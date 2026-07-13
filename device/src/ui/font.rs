use embedded_graphics::{
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_5X7, FONT_6X12, FONT_8X13},
    },
    pixelcolor::BinaryColor,
};



pub enum Font {
    Heading,
    Text,
    Description,
}

impl Font {
    pub fn get(&self) -> MonoTextStyle<'_, BinaryColor> {
        match self {
            Font::Heading => MonoTextStyleBuilder::new()
                .font(&FONT_8X13)
                .text_color(BinaryColor::On)
                .build(),
            Font::Text => MonoTextStyleBuilder::new()
                .font(&FONT_6X12)
                .text_color(BinaryColor::On)
                .build(),
            Font::Description => MonoTextStyleBuilder::new()
                .font(&FONT_5X7)
                .text_color(BinaryColor::On)
                .build(),
        }
    }

    pub fn width(&self) -> i32 {
        match self {
            Font::Heading => 8,
            Font::Text => 6,
            Font::Description => 5,
        }
    }

    pub fn height(&self) -> i32 {
        match self {
            Font::Heading => 13,
            Font::Text => 12,
            Font::Description => 7,
        }
    }
}
