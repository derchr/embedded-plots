use core::ops::{Range};

use crate::range_conv::Scalable;
use crate::drawable_curve::DrawableCurve;
use embedded_graphics::prelude::*;
use itertools::{Itertools, MinMaxResult::MinMax, MinMaxResult};

pub struct PlotPoint {
    pub x: i32,
    pub y: i32,
}

pub struct Curve<'a> {
    points: &'a [PlotPoint],
    pub x_range: Range<i32>,
    pub y_range: Range<i32>,
}

impl<'a> Curve<'a> {
    pub fn new(points: &'a [PlotPoint], x_range: Range<i32>, y_range: Range<i32>) -> Curve {
        Curve { points, x_range, y_range }
    }

    pub fn from_data(points: &'a [PlotPoint]) -> Curve {
        let x_range = match points
            .iter()
            .map(|p| (p.x))
            .minmax() {
            MinMaxResult::NoElements => 0..0,
            MinMaxResult::OneElement(v) => v..v,
            MinMax(min, max) => min..max,
        };

        let y_range = match points.iter().map(|p| (p.y)).minmax() {
            MinMaxResult::NoElements => 0..0,
            MinMaxResult::OneElement(v) => v..v,
            MinMax(min, max) => min..max,
        };

        Curve { points, x_range, y_range }
    }

    pub fn into_drawable_curve<C>(&self,
                                  top_left: &'a Point,
                                  bottom_right: &'a Point,
                                  color: C,
    ) -> DrawableCurve<C, impl Iterator<Item=Point> + '_>
        where C: PixelColor
    {
        assert!(top_left.x < bottom_right.x);
        assert!(top_left.y < bottom_right.y);
        assert!(!self.x_range.is_empty());
        assert!(!self.y_range.is_empty());

        let it = self.points.iter()
            .map(move |p| Point {
                x: p.x.scale_between_ranges(
                    &self.x_range,
                    &Range { start: top_left.x, end: bottom_right.x },
                ),
                y: p.y.scale_between_ranges(
                    &self.y_range,
                    &Range { start: bottom_right.y, end: top_left.y },
                ),
            });
        DrawableCurve::new(it, color)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
