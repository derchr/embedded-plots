use core::ops::{Range};

use crate::range_conv::Scalable;
use crate::drawable_curve::DrawableCurve;
use embedded_graphics::prelude::*;

pub struct PlotPoint {
    pub x: i32,
    pub y: i32,
}

pub struct Curve<'a>{
    points: &'a [PlotPoint],
}

impl<'a> Curve<'a> {
    pub fn new(points: &'a [PlotPoint]) -> Curve {
        Curve {points}
    }

    pub fn into_drawable_curve<C>(self,
                                  x_range: &'a Range<i32>,
                                  y_range: &'a Range<i32>,
                                  top_left : &'a Point,
                                  bottom_right: &'a Point,
                                  color: C
    ) -> DrawableCurve<C,impl Iterator<Item=Point> + 'a>
        where C: PixelColor
    {
        assert!(top_left.x < bottom_right.x);
        assert!(top_left.y < bottom_right.y);
        assert!(!x_range.is_empty());
        assert!(!y_range.is_empty());

        let it = self.points.iter()
            .map(move |p| Point{
                x: p.x.scale_between_ranges(
                    x_range,
                    &Range{start: top_left.x, end: bottom_right.x}
                ),
                y: p.y.scale_between_ranges(
                    y_range,
                    &Range{start: bottom_right.y, end: top_left.y}
                ),
            });
        DrawableCurve::new(it,color)
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
