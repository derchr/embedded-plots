use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_graphics_simulator::{SimulatorDisplay, Window, OutputSettingsBuilder};

use embedded_plots::Plot;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    let data = vec![
        Point::new(100, 100),
        Point::new(150, 100),
        Point::new(200, 200)];
    Plot::new(data.as_slice()
        ,RgbColor::GREEN)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}