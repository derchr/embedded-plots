use embedded_graphics::drawable::{Drawable};
use embedded_graphics::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{PixelColor};
use embedded_graphics::primitives::{Line, Primitive};
use embedded_graphics::style::PrimitiveStyle;

pub struct DrawableCurve<C, I>
{
    scaled_data: I,
    color: C,
    thickness: usize,
}

impl<C, I> DrawableCurve<C, I>
    where
        C: PixelColor,
        I: Iterator<Item=Point>,
{
    pub(in crate) fn new(data: I, color: C, thickness: usize) -> DrawableCurve<C, I> {
        DrawableCurve {
            scaled_data: data,
            color,
            thickness,
        }
    }
}

impl<C, I> Drawable<C> for DrawableCurve<C, I>
    where C: PixelColor,
          I: Iterator<Item=Point>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let style = PrimitiveStyle::with_stroke(self.color, self.thickness as u32);
        let mut iter = self.scaled_data.into_iter();
        let mut prev = iter.next().unwrap();
        for point in iter {
            Line::new(prev, point)
                .into_styled(style)
                .draw(display)?;
            prev = point;
        }
        Ok(())
    }
}