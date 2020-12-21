use crate::curve::Curve;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::pixelcolor::PixelColor;

pub struct PolyPlot<'a, C>
{
    curves: &'a [(Curve<'a>, C)],
    top_left: Point,
    bottom_right: Point,
}

impl<'a, C> PolyPlot<'a, C>
    where
        C: PixelColor
{
    pub fn new(curves: &'a [(Curve<'a>, C)], top_left: Point, bottom_right: Point) -> PolyPlot<C> {
        PolyPlot { curves, top_left, bottom_right }
    }
}

impl<'a, C> Drawable<C> for PolyPlot<'a, C>
    where
        C: PixelColor
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        for (curve, color) in self.curves {
            curve.into_drawable_curve(
                &self.top_left,
                &self.bottom_right,
                *color,
            ).draw(display)?;
        }
        Ok(())
    }
}