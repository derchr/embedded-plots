use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};

use embedded_graphics_simulator::{
    SimulatorDisplay,
    Window,
    OutputSettingsBuilder
};

use embedded_plots::curve::{PlotPoint, Curve};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    let data = vec![
        PlotPoint{x: 0,y: 0},
        PlotPoint{x: 1,y: 1},
        PlotPoint{x: 2,y: 1},
        PlotPoint{x: 3,y: 0},
    ];
    Curve::new(data.as_slice())
        .into_drawable_curve(&(0..3),&(0..1),&Point{x: 20, y: 20}, &Point{x:450,y:250},RgbColor::WHITE)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}