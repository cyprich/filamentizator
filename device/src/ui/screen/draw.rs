use embedded_graphics::{
    Drawable,
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use heapless::{String, Vec};

use crate::{
    MAX_FILAMENT_COUNT, MAX_STRING_LENGTH,
    display::Display,
    models::Filament,
    trunc_str,
    ui::{CHEVRON_DOWN, CHEVRON_LEFT, CHEVRON_RIGHT, CHEVRON_UP, FAVICON, Font},
};

use core::fmt::Write;

pub(super) async fn draw_welcome(display: &mut Display<'_>) {
    let mut y = 0;

    let welcome = Text::with_text_style(
        "Welcome to",
        Point::new(64, y),
        Font::Text.get(),
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build(),
    );

    y += Font::Text.height();

    let fila = Text::with_text_style(
        "Filamentizator",
        Point::new(64, y),
        Font::Heading.get(),
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build(),
    );

    const IMGSIZE: u32 = 16;
    let img = ImageRaw::<BinaryColor>::new(FAVICON, IMGSIZE);
    let img = Image::with_center(&img, Point::new(64, 64 - IMGSIZE as i32 / 2));

    welcome.draw(display).unwrap();
    fila.draw(display).unwrap();
    img.draw(display).unwrap();
}

pub(super) async fn draw_filaments(
    display: &mut Display<'_>,
    filaments: &Vec<Filament, MAX_FILAMENT_COUNT>,
    current_page: i32,
    max_page: i32,
) {
    let mut y = 0;

    // page number
    let mut page = String::<16>::new();
    write!(&mut page, "Page {}/{}", current_page, max_page).unwrap();
    let page_text_width = page.chars().count() as i32 * (Font::Description.width());
    let page = Text::with_text_style(
        &page,
        Point::new(128, 0),
        Font::Description.get(),
        TextStyleBuilder::new()
            .alignment(Alignment::Right)
            .baseline(Baseline::Top)
            .build(),
    );

    // border around page
    let page_rect_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .fill_color(BinaryColor::Off)
        .build();
    const PAD: usize = 2; // padding
    let page_rect = Rectangle::new(
        Point::new(128 - page_text_width - PAD as i32 - 1, -(PAD as i32)),
        Size::new(
            page_text_width as u32 + PAD as u32 * 2,
            Font::Description.height() as u32 + PAD as u32 * 2,
        ),
    )
    .into_styled(page_rect_style);

    // render each filament
    for f in filaments {
        let name = Text::with_baseline(&f.name, Point::new(0, y), Font::Text.get(), Baseline::Top);
        y += Font::Text.height();

        const LEN: usize = MAX_STRING_LENGTH * 2;
        let mut desc = String::<LEN>::new();
        write!(&mut desc, "{} - {}", f.vendor_name, f.material_name).unwrap();
        if let Some(color) = f.colors.first() {
            write!(&mut desc, " - {}", color).unwrap();
        }

        let desc = trunc_str(&desc, (120 / Font::Description.width()) as usize, 1);

        let desc = Text::with_baseline(
            &desc,
            Point::new(0, y),
            Font::Description.get(),
            Baseline::Top,
        );
        y += Font::Description.height();
        y += 3;

        name.draw(display).unwrap();
        desc.draw(display).unwrap();
    }

    page_rect.draw(display).unwrap();
    page.draw(display).unwrap();
    // page_line_horizontal.draw(display).unwrap();
    // page_line_vertical.draw(display).unwrap();
}

pub(super) async fn draw_navigation_help(display: &mut Display<'_>) {
    // fine-tuned values to make it look good
    const TEXT_ML: i32 = 12; // text margin left 
    const IMG_MT: i32 = 3; // image margin top
    const GAP_Y: i32 = 9;

    let mut y = 0;

    let title = Text::with_text_style(
        "Navigation",
        Point::new(64, y),
        Font::Text.get(),
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build(),
    );

    y += Font::Text.height();

    let chevron_right = ImageRaw::<BinaryColor>::new(CHEVRON_RIGHT, 8);
    let chevron_right = Image::new(&chevron_right, Point::new(0, y + IMG_MT));
    let chevron_right_text = Text::with_baseline(
        "Confirm / Select",
        Point::new(TEXT_ML, y),
        Font::Text.get(),
        Baseline::Top,
    );
    y += GAP_Y;

    let chevron_up = ImageRaw::<BinaryColor>::new(CHEVRON_UP, 8);
    let chevron_up = Image::new(&chevron_up, Point::new(0, y + IMG_MT));
    let chevron_up_text = Text::with_baseline(
        "Previous Item",
        Point::new(TEXT_ML, y),
        Font::Text.get(),
        Baseline::Top,
    );
    y += GAP_Y;

    let chevron_down = ImageRaw::<BinaryColor>::new(CHEVRON_DOWN, 8);
    let chevron_down = Image::new(&chevron_down, Point::new(0, y + IMG_MT));
    let chevron_down_text = Text::with_baseline(
        "Next Item",
        Point::new(TEXT_ML, y),
        Font::Text.get(),
        Baseline::Top,
    );
    y += GAP_Y;

    let chevron_left = ImageRaw::<BinaryColor>::new(CHEVRON_LEFT, 8);
    let chevron_left = Image::new(&chevron_left, Point::new(0, y + IMG_MT));
    let chevron_left_text = Text::with_baseline(
        "Cancel / Go Back",
        Point::new(TEXT_ML, y),
        Font::Text.get(),
        Baseline::Top,
    );

    let y = 64 - Font::Description.height();
    let line = Line::new(Point::new(0, y - 3), Point::new(128, y - 3))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
    let action = Text::with_baseline(
        "< Exit | Start >",
        Point::new(0, y),
        Font::Description.get(),
        Baseline::Top,
    );

    title.draw(display).unwrap();

    chevron_right.draw(display).unwrap();
    chevron_up.draw(display).unwrap();
    chevron_down.draw(display).unwrap();
    chevron_left.draw(display).unwrap();

    chevron_right_text.draw(display).unwrap();
    chevron_up_text.draw(display).unwrap();
    chevron_down_text.draw(display).unwrap();
    chevron_left_text.draw(display).unwrap();

    line.draw(display).unwrap();
    action.draw(display).unwrap();
}

pub(super) async fn draw_info(display: &mut Display<'_>, info: &str) {
    let mut y = 0;

    let header = Text::with_text_style(
        "Filamentizator",
        Point::new(64, y),
        Font::Description.get(),
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build(),
    );

    y += Font::Description.height() + 2;

    // let info = trunc_str(info, 3, 3);
    let info = trunc_str(info, 128 / Font::Text.width() as usize, 3);
    let text = Text::with_text_style(
        &info,
        Point::new(64, y),
        Font::Text.get(),
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build(),
    );

    const IMGSIZE: u32 = 16;
    let img = ImageRaw::<BinaryColor>::new(FAVICON, IMGSIZE);
    let img = Image::with_center(&img, Point::new(64, 64 - IMGSIZE as i32 / 2));

    header.draw(display).unwrap();
    text.draw(display).unwrap();
    img.draw(display).unwrap();
}

pub(super) async fn draw_error(display: &mut Display<'_>, error: &crate::Error) {
    let mut y = 0;

    let main = Text::with_text_style(
        error.get_type(),
        Point::new(64, y),
        Font::Heading.get(),
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build(),
    );
    y += Font::Heading.height();

    let desc = error.get_description();
    let desc = crate::trunc_str(&desc, (128 / Font::Text.width()) as usize, 3);
    let desc = Text::with_text_style(
        &desc,
        Point::new(64, y),
        Font::Text.get(),
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build(),
    );

    main.draw(display).unwrap();
    desc.draw(display).unwrap();

    if let Some(val) = error.get_hint() {
        let mut hint = String::<64>::new();
        write!(&mut hint, "Hint:\n{}", val).unwrap();

        let hint = Text::with_text_style(
            &hint,
            Point::new(0, 64 - Font::Description.height()),
            Font::Description.get(),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Bottom)
                .build(),
        );

        hint.draw(display).unwrap();
    }
}
