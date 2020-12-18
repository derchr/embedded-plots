use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};

use embedded_graphics_simulator::{
    SimulatorDisplay,
    Window,
    OutputSettingsBuilder
};

use embedded_plots::{
    CurvePoints,
    PlotPoint
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    let data = vec![
        PlotPoint{x: 100,y: 100},
        PlotPoint{x: 150,y: 100},
        PlotPoint{x: 200,y: 200}];
    CurvePoints::new(data.as_slice())
        .into_drawable_curve(&(100..200),&(100..200),Point{x: 00, y: 0}, Point{x:480,y:272},RgbColor::WHITE)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}