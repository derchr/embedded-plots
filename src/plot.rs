use crate::curve::Curve;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::pixelcolor::PixelColor;

struct Plot<'a, C>
    where
        C: PixelColor
{
    curves: &'a [Curve<'a>],
    color: C,
}

impl<'a, C> Plot<'a, C> {
    fn new(curves: &'a [Curve<'a>], color : C) -> Plot<C> {
        Plot{curves, color}
    }
}

impl<C> Drawable<C> for Plot<'_, C>
    where
        C: PixelColor
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), <D as DrawTarget<_>>::Error> {
        for curve in self.curves {
            curve.into_drawable_curve(
                &(0..50),
                &(-10..10),
                &Point{x: 10,y: 10},
                &Point{x: 470, y: 270},
                self.color
            ).draw(display)?;
        }
        Ok(())
    }
}