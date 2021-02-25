use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    style::TextStyleBuilder,
    fonts::{Font6x8, Font6x6},
};

use embedded_graphics_simulator::{
    SimulatorDisplay,
    Window,
    OutputSettingsBuilder,
};

use embedded_plots::{
    axis::{Axis, Placement, Scale},
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    let text_style_white = TextStyleBuilder::new(Font6x8)
        .text_color(RgbColor::WHITE)
        .build();

    let text_style_yellow_compact = TextStyleBuilder::new(Font6x6)
        .text_color(RgbColor::YELLOW)
        .build();

    Axis::new("X Fixed 0-100(10)",  0..100, Scale::Fixed(10))
        .into_drawable_axis(
            Placement::X{x1: 40, x2: 230, y: 10},
            RgbColor::WHITE,
            text_style_white,
            2,
            2,
        )
        .draw(&mut display)?;

    Axis::new("X Fixed 0-200(100)",  0..200, Scale::Fixed(100))
        .into_drawable_axis(
            Placement::X{x1: 240, x2: 470, y: 10},
            RgbColor::YELLOW,
            text_style_yellow_compact,
            1,
            2,
        )
        .draw(&mut display)?;

    Axis::new("X Frac 0-100(7)",  0..100, Scale::RangeFraction(7))
        .into_drawable_axis(
            Placement::X{x1: 50, x2: 220, y: 30},
            RgbColor::BLUE,
            text_style_white,
            3,
            1,
        )
        .draw(&mut display)?;

    Axis::new("X Frac 0-200(4)",  0..200, Scale::RangeFraction(4))
        .into_drawable_axis(
            Placement::X{x1: 250, x2: 460, y: 40},
            RgbColor::RED,
            text_style_yellow_compact,
            7,
            1,
        )
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-100(10)",  0..100, Scale::Fixed(10))
        .into_drawable_axis(
            Placement::Y{y1: 70, y2: 230, x: 160},
            RgbColor::WHITE,
            text_style_white,
            2,
            1,
        )
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-200(100)",  0..200, Scale::Fixed(100))
        .into_drawable_axis(
            Placement::Y{y1: 70, y2: 210, x: 260},
            RgbColor::YELLOW,
            text_style_yellow_compact,
            1,
            1,
        )
        .draw(&mut display)?;

    Axis::new("Y Frac 0-100(7)",  0..100, Scale::RangeFraction(7))
        .into_drawable_axis(
            Placement::Y{y1: 60, y2: 180, x: 370},
            RgbColor::BLUE,
            text_style_white,
            3,
            1,
        )
        .draw(&mut display)?;

    Axis::new("Y Frac 0-200(4)",  0..200, Scale::RangeFraction(4))
        .into_drawable_axis(
            Placement::Y{y1: 90, y2: 220, x: 470},
            RgbColor::RED,
            text_style_yellow_compact,
            7,
            1,
        )
        .draw(&mut display)?;

    Axis::new("X",  123..2137, Scale::Fixed(150))
        .into_drawable_axis(
            Placement::X{x1: 30, x2: 470, y: 250},
            RgbColor::YELLOW,
            text_style_white,
            2,
            1,
        )
        .draw(&mut display)?;

    Axis::new("Y",  0..2137, Scale::RangeFraction(15))
        .into_drawable_axis(
            Placement::Y{y1: 10, y2: 250, x: 30},
            RgbColor::WHITE,
            text_style_white,
            2,
            1,
        )
        .draw(&mut display)?;


    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(1)
        .build();
    Window::new("Free axis", &output_settings).show_static(&display);

    Ok(())
}