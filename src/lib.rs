// #![no_std]

mod range_conv;

use embedded_graphics::drawable::{Drawable};
use embedded_graphics::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{PixelColor};
use embedded_graphics::primitives::{Line, Primitive};
use embedded_graphics::style::PrimitiveStyle;

use crate::range_conv::Scalable;
use core::ops::{Range};

pub struct PlotPoint {
    pub x: i32,
    pub y: i32,
}

pub struct CurvePoints<'a>{
    points: &'a [PlotPoint],
}

impl<'a> CurvePoints<'a> {
    pub fn new(points: &'a [PlotPoint]) -> CurvePoints {
        CurvePoints{points}
    }

    pub fn into_drawable_curve<C>(self, x_range: &Range<i32>, y_range: &Range<i32>, top_left : Point, bottom_right: Point, color: C) -> DrawableCurve<C>
    where C: PixelColor
    {
        assert!(top_left.x < bottom_right.x);
        assert!(top_left.y < bottom_right.y);
        assert!(!x_range.is_empty());
        assert!(!y_range.is_empty());

        let vec = self.points.iter()
            .map(|p| Point{
                x: p.x.scale_between_ranges(x_range,&Range{start: top_left.x.into(), end: bottom_right.x.into()}).into(),
                y: p.y.scale_between_ranges(y_range,&Range{start: top_left.y.into(), end: bottom_right.y.into()}).into(),
            })
            .collect();
        DrawableCurve::new(vec,color)
    }
}

pub struct DrawableCurve<C>
{
    scaled_and_pos_data: Vec<Point>,
    color: C,
}

impl<'a, C> DrawableCurve<C>
    where C: PixelColor
{
    pub fn new(data: Vec<Point>,color : C) -> DrawableCurve<C> {
        println!("data: {:?}",data);
        DrawableCurve {
            scaled_and_pos_data: data,
            color,
        }
    }
}

impl<'a, C> Drawable<C> for DrawableCurve<C>
    where C: PixelColor
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), <D as DrawTarget<C>>::Error> {
        let style = PrimitiveStyle::with_stroke(self.color,2);
        for i in 1..self.scaled_and_pos_data.len() {
            Line::new(self.scaled_and_pos_data[i-1],self.scaled_and_pos_data[i]).into_styled(style).draw(display)?;
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
