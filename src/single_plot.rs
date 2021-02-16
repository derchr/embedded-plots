use crate::curve::Curve;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::pixelcolor::PixelColor;
use crate::axis::{Scale, Placement, Axis};
use embedded_graphics::style::TextStyleBuilder;
use embedded_graphics::fonts::Font6x8;

pub struct SinglePlot<'a> {
    curve: &'a Curve<'a>,
    x_scale: Scale,
    y_scale: Scale,
}

impl<'a> SinglePlot<'a> {
    pub fn new(curve: &'a Curve<'a>, x_scale: Scale, y_scale: Scale) -> SinglePlot {
        SinglePlot { curve, x_scale, y_scale }
    }

    pub fn into_drawable<C: PixelColor>(self, color: C, top_left: Point, bottom_right: Point) -> DrawableSinglePlot<'a, C> {
        DrawableSinglePlot { plot: self, color, top_left, bottom_right }
    }
}

pub struct DrawableSinglePlot<'a, C>
    where
        C: PixelColor
{
    plot: SinglePlot<'a>,
    color: C,
    top_left: Point,
    bottom_right: Point,
}

impl<'a, C> Drawable<C> for DrawableSinglePlot<'a, C>
    where
        C: PixelColor
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let text_style = TextStyleBuilder::new(Font6x8)
            .text_color(self.color)
            .build();

        Axis::new("X", self.plot.curve.x_range.clone(), self.plot.x_scale)
            .into_drawable_axis(Placement::X { x1: self.top_left.x, x2: self.bottom_right.x, y: self.bottom_right.y }, self.color, text_style, 2)
            .draw(display)?;

        Axis::new("Y", self.plot.curve.y_range.clone(), self.plot.y_scale)
            .into_drawable_axis(Placement::Y { y1: self.top_left.y, y2: self.bottom_right.y, x: self.top_left.x }, self.color, text_style, 2)
            .draw(display)?;

        self.plot.curve.into_drawable_curve(
            &self.top_left,
            &self.bottom_right,
            self.color,
        ).draw(display)?;
        Ok(())
    }
}