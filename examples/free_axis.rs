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

    Axis::new("X Fixed 0-100(10)", Placement::X{x1: 40, x2: 230, y: 10}, 0..100, Scale::Fixed(10), RgbColor::WHITE, text_style_white, 2)
        .draw(&mut display)?;

    Axis::new("X Fixed 0-200(100)", Placement::X{x1: 240, x2: 470, y: 10}, 0..200, Scale::Fixed(100), RgbColor::YELLOW, text_style_yellow_compact, 1)
        .draw(&mut display)?;

    Axis::new("X Frac 0-100(7)", Placement::X{x1: 50, x2: 220, y: 30}, 0..100, Scale::RangeFraction(7), RgbColor::BLUE, text_style_white, 3)
        .draw(&mut display)?;

    Axis::new("X Frac 0-200(4)", Placement::X{x1: 250, x2: 460, y: 40}, 0..200, Scale::RangeFraction(4), RgbColor::RED, text_style_yellow_compact, 7)
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-100(10)", Placement::Y{y1: 70, y2: 230, x: 160}, 0..100, Scale::Fixed(10), RgbColor::WHITE, text_style_white, 2)
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-200(100)", Placement::Y{y1: 70, y2: 210, x: 260}, 0..200, Scale::Fixed(100), RgbColor::YELLOW, text_style_yellow_compact, 1)
        .draw(&mut display)?;

    Axis::new("Y Frac 0-100(7)", Placement::Y{y1: 60, y2: 180, x: 370}, 0..100, Scale::RangeFraction(7), RgbColor::BLUE, text_style_white, 3)
        .draw(&mut display)?;

    Axis::new("Y Frac 0-200(4)", Placement::Y{y1: 90, y2: 220, x: 470}, 0..200, Scale::RangeFraction(4), RgbColor::RED, text_style_yellow_compact, 7)
        .draw(&mut display)?;

    Axis::new("X", Placement::X{x1: 30, x2: 470, y: 250}, 123..2137, Scale::Fixed(150), RgbColor::YELLOW, text_style_white, 2)
        .draw(&mut display)?;
    Axis::new("Y", Placement::Y{y1: 10, y2: 250, x: 30}, 0..2137, Scale::RangeFraction(15), RgbColor::WHITE, text_style_white, 2)
        .draw(&mut display)?;


    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(1)
        .build();
    Window::new("Basic plot", &output_settings).show_static(&display);

    Ok(())
}