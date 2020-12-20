use crate::curve::Curve;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::pixelcolor::PixelColor;

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

        self.curve.into_drawable_curve(
            &self.top_left,
            &self.bottom_right,
            self.color,
        ).draw(display)?;
        Ok(())
    }
}