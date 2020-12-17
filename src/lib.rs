#![no_std]

use embedded_graphics::drawable::{Drawable};
use embedded_graphics::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{PixelColor};
use embedded_graphics::primitives::{Line, Primitive};
use embedded_graphics::style::PrimitiveStyle;

pub struct Plot<'a, C>
{
    data: &'a [Point],
    color: C,
}

impl<'a, C> Plot<'a, C>
    where C: PixelColor
{
    pub fn new(data: &'a [Point],color : C) -> Plot<C> {
        Plot {
            data,
            color,
        }
    }
}

impl<'a, C> Drawable<C> for Plot<'a, C>
    where C: PixelColor
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), <D as DrawTarget<C>>::Error> {
        let style = PrimitiveStyle::with_stroke(self.color,2);
        for i in 1..self.data.len() {
            Line::new(self.data[i-1],self.data[i]).into_styled(style).draw(display)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
