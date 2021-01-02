use crate::curve::Curve;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::pixelcolor::PixelColor;
use crate::axis::{Scale, Placement, Axis};
use embedded_graphics::style::TextStyleBuilder;
use embedded_graphics::fonts::Font6x8;

pub struct SinglePlot<'a, C>
    where
        C: PixelColor
{
    curve: &'a Curve<'a>,
    color: C,
    top_left: Point,
    bottom_right: Point,
}

impl<'a, C> SinglePlot<'a, C>
    where
        C: PixelColor
{
    pub fn new(curve: &'a Curve<'a>, color: C, top_left: Point, bottom_right: Point) -> SinglePlot<'a, C> {
        SinglePlot { curve, color, top_left, bottom_right}
    }
}

impl<'a, C> Drawable<C> for SinglePlot<'a, C>
    where
        C: PixelColor
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {

        let text_style = TextStyleBuilder::new(Font6x8)
            .text_color(self.color)
            .build();

        Axis::new("X", Placement::X{x1: self.top_left.x, x2: self.bottom_right.x, y: self.bottom_right.y}, 0..100, Scale::Fixed(10), self.color, text_style, 2)
            .draw(display)?;

        Axis::new("Y", Placement::Y{y1: self.top_left.y, y2: self.bottom_right.y, x: self.top_left.x}, 0..200, Scale::Fixed(10), self.color, text_style, 1)
            .draw(display)?;

        self.curve.into_drawable_curve(
            &self.top_left,
            &self.bottom_right,
            self.color,
        ).draw(display)?;
        Ok(())
    }
}