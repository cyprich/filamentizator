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
    selected_filament: i32,
) {
    let x = 6;
    let mut y = 0;
    const GAP: i32 = 4;

    // page number
    let mut page = String::<16>::new();
    write!(&mut page, "{}/{}", current_page, max_page).unwrap();
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
        // create name
        let name = Text::with_baseline(&f.name, Point::new(x, y), Font::Text.get(), Baseline::Top);
        y += Font::Text.height();

        // format description
        const LEN: usize = MAX_STRING_LENGTH * 2;
        let mut desc = String::<LEN>::new();
        write!(&mut desc, "{} - {}", f.vendor_name, f.material_name).unwrap();
        if let Some(color) = f.colors.first() {
            write!(&mut desc, " - {}", color).unwrap();
        }

        // create description
        let desc = trunc_str(&desc, ((128 - x) / Font::Description.width()) as usize, 1);
        let desc = Text::with_baseline(
            &desc,
            Point::new(x, y),
            Font::Description.get(),
            Baseline::Top,
        );
        y += Font::Description.height();
        y += GAP;

        name.draw(display).unwrap();
        desc.draw(display).unwrap();
    }

    // indicator
    let x = -3; // slightly over the edge - smaller and creates a space
    y = 8; // constant (offset from the top)
    y += (selected_filament - 1) * (Font::Text.height() + Font::Description.height() + GAP); // line
    let indi = ImageRaw::<BinaryColor>::new(CHEVRON_RIGHT, 8);
    let indi = Image::new(&indi, Point::new(x, y));

    page_rect.draw(display).unwrap();
    page.draw(display).unwrap();
    indi.draw(display).unwrap();
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

pub(super) async fn draw_filament(display: &mut Display<'_>, f: &Filament) {
    let mut y = 0;
    let h = Font::Description.height() + 1; // line height
    const LEN: usize = MAX_STRING_LENGTH + 16;

    // title
    let name = trunc_str(&f.name, (128 / Font::Text.width()) as usize, 1);
    let name = Text::with_text_style(
        &name,
        Point::new(64, 0),
        Font::Text.get(),
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build(),
    );
    y += Font::Text.height() + 2;

    // vendor
    let mut vendor = String::<LEN>::new();
    write!(&mut vendor, "Vendor: {}", f.vendor_name).unwrap();
    let vendor = trunc_str(&vendor, 128 / Font::Description.width() as usize, 1);
    let vendor = Text::with_baseline(
        &vendor,
        Point::new(0, y),
        Font::Description.get(),
        Baseline::Top,
    );
    y += h;

    // material
    let mut material = String::<LEN>::new();
    write!(&mut material, "Material: {}", f.material_name).unwrap();
    let material = trunc_str(&material, 128 / Font::Description.width() as usize, 1);
    let material = Text::with_baseline(
        &material,
        Point::new(0, y),
        Font::Description.get(),
        Baseline::Top,
    );
    y += h;

    // price
    let mut price = String::<LEN>::new();
    write!(&mut price, "Price: {}", f.price).unwrap();
    let price = trunc_str(&price, 128 / Font::Description.width() as usize, 1);
    let price = Text::with_baseline(
        &price,
        Point::new(0, y),
        Font::Description.get(),
        Baseline::Top,
    );
    y += h;

    // weight
    let mut weight = String::<LEN>::new();
    write!(
        &mut weight,
        "Weight: {} (+Spool {})",
        f.weight_net, f.weight_spool
    )
    .unwrap();
    let weight = trunc_str(&weight, 128 / Font::Description.width() as usize, 1);
    let weight = Text::with_baseline(
        &weight,
        Point::new(0, y),
        Font::Description.get(),
        Baseline::Top,
    );
    y += h;

    // temp
    let mut temp = String::<LEN>::new();
    write!(&mut temp, "Temp: {}", f.temp_min).unwrap();
    if let Some(val) = f.temp_max {
        write!(&mut temp, "-{}", val).unwrap()
    }
    write!(&mut temp, " | Bed: {}", f.temp_bed_min).unwrap();
    if let Some(val) = f.temp_bed_max {
        write!(&mut temp, "-{}", val).unwrap();
    }
    let temp = trunc_str(&temp, 128 / Font::Description.width() as usize, 1);
    let temp = Text::with_baseline(
        &temp,
        Point::new(0, y),
        Font::Description.get(),
        Baseline::Top,
    );

    y = 64 - Font::Description.height();
    // footer
    let foot_line = Line::new(Point::new(0, y - 2), Point::new(128, y - 2))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
    let foot = Text::with_baseline(
        "< Filaments | Log >",
        Point::new(0, y),
        Font::Description.get(),
        Baseline::Top,
    );

    // drawing
    name.draw(display).unwrap();
    vendor.draw(display).unwrap();
    material.draw(display).unwrap();
    price.draw(display).unwrap();
    weight.draw(display).unwrap();
    temp.draw(display).unwrap();
    foot_line.draw(display).unwrap();
    foot.draw(display).unwrap();
}
