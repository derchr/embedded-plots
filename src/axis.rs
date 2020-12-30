use embedded_graphics::primitives::Line;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::DrawTarget;
use core::ops::Range;
use embedded_graphics::prelude::*;
use embedded_graphics::style::{PrimitiveStyle, TextStyle};
use crate::range_conv::Scalable;
use embedded_graphics::fonts::Text;
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

pub struct Axis<'a, C, F>
    where
        C: PixelColor,
        F: Font,
        TextStyle<C,F>: Clone,
{
    title: &'a str,
    orientation: Orientation,
    range: Range<i32>,
    scale: Scale,
    color: C,
    text_style: TextStyle<C,F>
}

impl<'a, C, F> Axis<'a, C, F>
    where
        C: PixelColor,
        F: Font,
        TextStyle<C,F>: Clone,
{
    pub fn new(title: &'a str, orientation: Orientation, range: Range<i32>, scale: Scale, color: C, text_style: TextStyle<C,F>) -> Axis<'a, C, F> {
        Axis { title, orientation, range, scale, color, text_style }
    }
}


impl<'a, C, F> Drawable<C> for Axis<'a, C, F>
    where
        C: PixelColor,
        F: Font + Copy,
        TextStyle<C,F>: Clone,
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
        match self.orientation {
            Orientation::X { x1, x2, y } => {
                Line { start: Point { x: x1, y }, end: Point { x: x2, y } }
                    .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                    .draw(display)?;
                let title = Text::new(self.title, Point { x: x1, y: y + 10 })
                    .into_styled(self.text_style);
                let title = title.translate(Point { x: (x2 - x1) / 2 - title.size().width as i32 / 2, y: 0 });
                title.draw(display)?;

                for line in lines {
                    let x = line.scale_between_ranges(&self.range, &(x1..x2));
                    Line { start: Point { x, y: y - 2 }, end: Point { x, y: y + 2 } }
                        .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                        .draw(display)?;
                    let mut buf: String::<U8> = String::new();
                    write!(buf, "{}", line).unwrap();
                    Text::new(&buf, Point { x: x + 1, y: y + 1 }).into_styled(self.text_style).draw(display)?;
                }
            }
            Orientation::Y { y1, y2, x } => {
                Line { start: Point { x, y: y1 }, end: Point { x, y: y2 } }
                    .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                    .draw(display)?;
                let title = Text::new(self.title, Point { x, y: y1 })
                    .into_styled(self.text_style);
                let title = title.translate(Point { x: -(title.size().width as i32) - 5, y: (y2-y1)/2 });
                title.draw(display)?;

                for line in lines {
                    let y = line.scale_between_ranges(&self.range, &(y2..y1));
                    Line { start: Point { x: x - 2, y }, end: Point { x: x + 2, y} }
                        .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                        .draw(display)?;
                    let mut buf: String::<U8> = String::new();
                    write!(buf, "{}", line).unwrap();
                    let tick = Text::new(&buf, Point { x, y}).into_styled(self.text_style);
                    let tick = tick.translate(Point{ x: -(tick.size().width as i32), y: 0 });
                    tick.draw(display)?;
                }
            }
        }
        Ok(())
    }
}