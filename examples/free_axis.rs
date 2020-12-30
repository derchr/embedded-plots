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

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    Axis::new("X Fixed 0-100(10)",Orientation::X{x1: 10, x2: 230, y: 10},0..100,Scale::Fixed(10),RgbColor::WHITE)
        .draw(&mut display)?;

    Axis::new("X Fixed 0-200(100)",Orientation::X{x1: 240, x2: 470, y: 10},0..200,Scale::Fixed(100),RgbColor::YELLOW)
        .draw(&mut display)?;

    Axis::new("X Fixed 0-100(7)",Orientation::X{x1: 20, x2: 220, y: 30},0..100,Scale::RangeFraction(7),RgbColor::BLUE)
        .draw(&mut display)?;

    Axis::new("X Fixed 0-200(4)",Orientation::X{x1: 250, x2: 460, y: 30},0..200,Scale::RangeFraction(4),RgbColor::RED)
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-100(10)",Orientation::Y{y1: 60, y2: 250, x: 90},0..100,Scale::Fixed(10),RgbColor::WHITE)
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-200(100)",Orientation::Y{y1: 70, y2: 230, x: 210},0..200,Scale::Fixed(100),RgbColor::YELLOW)
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-100(7)",Orientation::Y{y1: 60, y2: 180, x: 330},0..100,Scale::RangeFraction(7),RgbColor::BLUE)
        .draw(&mut display)?;

    Axis::new("Y Fixed 0-200(4)",Orientation::Y{y1: 90, y2: 260, x: 450},0..200,Scale::RangeFraction(4),RgbColor::RED)
        .draw(&mut display)?;


    let output_settings = OutputSettingsBuilder::new()
        // .pixel_spacing(1)
        .build();
    Window::new("Basic plot", &output_settings).show_static(&display);

    Ok(())
}