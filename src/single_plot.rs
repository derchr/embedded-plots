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
    x_scale: Scale,
    y_scale: Scale,
    color: C,
    top_left: Point,
    bottom_right: Point,
}

impl<'a, C> SinglePlot<'a, C>
    where
        C: PixelColor
{
    pub fn new(curve: &'a Curve<'a>, color: C, top_left: Point, bottom_right: Point, x_scale: Scale, y_scale: Scale) -> SinglePlot<'a, C> {
        SinglePlot { curve, color, top_left, bottom_right, x_scale, y_scale}
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

        Axis::new("X",self.curve.x_range.clone(),self.x_scale)
            .into_drawable_axis(Placement::X{x1: self.top_left.x, x2: self.bottom_right.x, y: self.bottom_right.y},self.color,text_style,2)
            .draw(display)?;

        Axis::new("Y",  self.curve.y_range.clone(), self.y_scale)
            .into_drawable_axis(Placement::Y{y1: self.top_left.y, y2: self.bottom_right.y, x: self.top_left.x},self.color,text_style,2)
            .draw(display)?;

        self.curve.into_drawable_curve(
            &self.top_left,
            &self.bottom_right,
            self.color,
        ).draw(display)?;
        Ok(())
    }
}