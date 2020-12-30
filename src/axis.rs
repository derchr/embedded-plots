use embedded_graphics::primitives::Line;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::DrawTarget;
use core::ops::Range;
use embedded_graphics::prelude::*;
use embedded_graphics::style::{PrimitiveStyle, TextStyleBuilder};
use crate::range_conv::Scalable;
use embedded_graphics::fonts::{Text, Font6x8};
use heapless::{consts::*, String};
use core::fmt::Write;

pub enum Orientation {
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

pub enum Scale {
    Fixed(usize),
    RangeFraction(usize),
}

pub struct Axis<'a, C> {
    title: &'a str,
    orientation: Orientation,
    range: Range<i32>,
    scale: Scale,
    color: C,
}

impl<'a, C> Axis<'a, C>
    where
        C: PixelColor,
{
    pub fn new(title: &'a str, orientation: Orientation, range: Range<i32>, scale: Scale, color: C) -> Axis<'a, C> {
        Axis { title, orientation, range, scale, color }
    }
}


impl<'a, C> Drawable<C> for Axis<'a, C>
    where
        C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let lines = match self.scale {
            Scale::Fixed(interval) => {
                self.range.clone().into_iter().step_by(interval)
            }
            Scale::RangeFraction(fraction) => {
                let len = self.range.len();
                self.range.clone().into_iter().step_by(len / fraction)
            }
        };
        // Create a new text style
        let style = TextStyleBuilder::new(Font6x8)
            .text_color(self.color)
            .build();
        match self.orientation {
            Orientation::X { x1, x2, y } => {
                Line { start: Point { x: x1, y }, end: Point { x: x2, y } }
                    .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                    .draw(display)?;
                let title = Text::new(self.title, Point { x: x1, y: y + 10 })
                    .into_styled(style);
                let title = title.translate(Point { x: (x2 - x1) / 2 - title.size().width as i32 / 2, y: 0 });
                title.draw(display)?;

                for line in lines {
                    let x = line.scale_between_ranges(&self.range, &(x1..x2));
                    Line { start: Point { x, y: y - 2 }, end: Point { x, y: y + 2 } }
                        .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                        .draw(display)?;
                    let mut buf: String::<U8> = String::new();
                    write!(buf, "{}", line).unwrap();
                    Text::new(&buf, Point { x: x + 1, y: y + 1 }).into_styled(style).draw(display)?;
                }
            }
            Orientation::Y { y1, y2, x } => {
                Line { start: Point { x, y: y1 }, end: Point { x, y: y2 } }
                    .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                    .draw(display)?;
                let title = Text::new(self.title, Point { x, y: y1 })
                    .into_styled(style);
                let title = title.translate(Point { x: -(title.size().width as i32) - 5, y: (y2-y1)/2 });
                title.draw(display)?;

                for line in lines {
                    let y = line.scale_between_ranges(&self.range, &(y2..y1));
                    Line { start: Point { x: x - 2, y }, end: Point { x: x + 2, y} }
                        .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                        .draw(display)?;
                    let mut buf: String::<U8> = String::new();
                    write!(buf, "{}", line).unwrap();
                    let tick = Text::new(&buf, Point { x, y}).into_styled(style);
                    let tick = tick.translate(Point{ x: -(tick.size().width as i32), y: 0 });
                    tick.draw(display)?;
                }
            }
        }
        Ok(())
    }
}