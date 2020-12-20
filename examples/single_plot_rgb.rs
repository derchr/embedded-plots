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
    single_plot::{SinglePlot},
    curve::{PlotPoint, Curve},
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    let data = vec![
        PlotPoint { x: 0, y: 0 },
        PlotPoint { x: 1, y: 1 },
        PlotPoint { x: 2, y: 1 },
        PlotPoint { x: 3, y: 0 },
    ];
    let curve = Curve::from_data(data.as_slice());

    let plot = SinglePlot::new(&curve, RgbColor::YELLOW, Point { x: 10, y: 10 }, Point { x: 470, y: 260 });

    plot.draw(&mut display)?;
    let output_settings = OutputSettingsBuilder::new()
        .build();
    Window::new("Basic plot", &output_settings).show_static(&display);

    Ok(())
}