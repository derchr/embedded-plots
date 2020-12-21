use embedded_graphics::primitives::Line;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::DrawTarget;
use core::ops::Range;
use embedded_graphics::prelude::*;
use embedded_graphics::style::{PrimitiveStyle};
use core::borrow::Borrow;

enum Orientation {
    X {
        x1: i32,
        x2: i32,
        y: i32,
    },
    Y {
        y1: i32,
        y2: i32,
        x: i32,
    },
}

enum Scale {
    Fixed {
        interval: usize,
    },
    RangeFraction {
        fraction: usize,
    },
}

pub struct Axis<C> {
    orientation: Orientation,
    range: Range<i32>,
    scale: Scale,
    color: C
}

impl<C> Axis<C>
    where
        C: PixelColor,
{
    fn new(orientation: Orientation, range: Range<i32>, scale: Scale, color: C) -> Axis<C> {
        Axis { orientation, range, scale, color }
    }
}


impl<C> Drawable<C> for Axis<C>
    where
        C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let lines = match self.scale {
            Scale::Fixed { interval } => {
                self.range.step_by(interval)
            }
            Scale::RangeFraction { fraction } => {
                let len = self.range.len();
                self.range.step_by(len / fraction)
            }
        };
        match self.orientation {
            Orientation::X{x1,x2,y} => {
                Line{start: Point{x: x1,y}, end: Point{x: x2,y}}
                    .into_styled(PrimitiveStyle::with_stroke(self.color,1))
                    .draw(display)?;
                for line in lines {
                    line.abs();
                }
            }
            Orientation::Y{y1,y2,x} => {
                Line{start: Point{x,y: y1}, end: Point{x,y: y2}}
                    .into_styled(PrimitiveStyle::with_stroke(self.color,1))
                    .draw(display)?;
                for line in lines {
                    line.abs();
                }
            }
        }
        Ok(())
    }
}