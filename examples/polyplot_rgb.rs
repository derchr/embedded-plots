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
    polyplot::{PolyPlot},
    curve::{PlotPoint, Curve},
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    let data1 = vec![
        PlotPoint { x: 0, y: 0 },
        PlotPoint { x: 1, y: 1 },
        PlotPoint { x: 2, y: 1 },
        PlotPoint { x: 3, y: 0 },
    ];

    let data2 = vec![
        PlotPoint { x: 0, y: 1 },
        PlotPoint { x: 1, y: 0 },
        PlotPoint { x: 2, y: 3 },
        PlotPoint { x: 3, y: 2 },
        PlotPoint { x: 4, y: 2 },
    ];

    let curves = vec![
        (Curve::from_data(data1.as_slice()), RgbColor::YELLOW),
        (Curve::from_data(data2.as_slice()), RgbColor::RED),
    ];

    let plot = PolyPlot::new(curves.as_slice(), Point { x: 10, y: 10 }, Point { x: 470, y: 260 });

    plot.draw(&mut display)?;
    let output_settings = OutputSettingsBuilder::new()
        .build();
    Window::new("Basic plot", &output_settings).show_static(&display);

    Ok(())
}