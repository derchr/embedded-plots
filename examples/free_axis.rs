use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};

use embedded_graphics_simulator::{
    SimulatorDisplay,
    Window,
    OutputSettingsBuilder,

};

use embedded_plots::{
    axis::{Axis,Orientation,Scale},
};
use embedded_graphics::style::TextStyleBuilder;
use embedded_graphics::fonts::Font6x8;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(RgbColor::WHITE)
        .build();

    Axis::new("X Fixed 0-100(10)",Orientation::X{x1: 10, x2: 230, y: 10},0..100,Scale::Fixed(10),RgbColor::WHITE, text_style)
        .draw(&mut display)?;

    Axis::new("X Fixed 0-200(100)",Orientation::X{x1: 240, x2: 470, y: 10},0..200,Scale::Fixed(100),RgbColor::YELLOW, text_style)
        .draw(&mut display)?;

    Axis::new("X Frac 0-100(7)",Orientation::X{x1: 20, x2: 220, y: 30},0..100,Scale::RangeFraction(7),RgbColor::BLUE, text_style)
        .draw(&mut display)?;

    Axis::new("X Frac 0-200(4)",Orientation::X{x1: 250, x2: 460, y: 30},0..200,Scale::RangeFraction(4),RgbColor::RED, text_style)
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-100(10)",Orientation::Y{y1: 60, y2: 250, x: 110},0..100,Scale::Fixed(10),RgbColor::WHITE, text_style)
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-200(100)",Orientation::Y{y1: 70, y2: 230, x: 230},0..200,Scale::Fixed(100),RgbColor::YELLOW, text_style)
        .draw(&mut display)?;

    Axis::new("Y Frac 0-100(7)",Orientation::Y{y1: 60, y2: 180, x: 350},0..100,Scale::RangeFraction(7),RgbColor::BLUE, text_style)
        .draw(&mut display)?;

    Axis::new("Y Frac 0-200(4)",Orientation::Y{y1: 90, y2: 260, x: 470},0..200,Scale::RangeFraction(4),RgbColor::RED, text_style)
        .draw(&mut display)?;


    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(1)
        .build();
    Window::new("Basic plot", &output_settings).show_static(&display);

    Ok(())
}