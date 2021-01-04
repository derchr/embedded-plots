use embedded_graphics::prelude::*;
use crate::axis::{Placement, Scale};
use core::ops::Range;
use embedded_graphics::style::{TextStyle, PrimitiveStyle};
use embedded_graphics::fonts::Text;
use crate::range_conv::Scalable;
use embedded_graphics::primitives::Line;
use heapless::{consts::*, String};
use core::fmt::Write;


pub struct DrawableAxis<'a, C, F>
    where
        C: PixelColor,
        F: Font,
        TextStyle<C, F>: Clone,
{
    title: &'a str,
    placement: Placement,
    range: Range<i32>,
    scale: Scale,
    color: C,
    text_style: TextStyle<C, F>,
    tick_size: usize,
}

impl<'a, C, F> DrawableAxis<'a, C, F>
    where
        C: PixelColor,
        F: Font,
        TextStyle<C, F>: Clone,
{
    pub(in crate) fn new(title: &'a str, placement: Placement, range: Range<i32>, scale: Scale, color: C, text_style: TextStyle<C, F>, tick_height: usize) -> DrawableAxis<'a, C, F> {
        DrawableAxis { title, placement, range, scale, color, text_style, tick_size: tick_height }
    }

    pub fn size(&self) -> Point {
        Point { x: 50, y: 50 }
    }
}


impl<'a, C, F> Drawable<C> for DrawableAxis<'a, C, F>
    where
        C: PixelColor,
        F: Font + Copy,
        TextStyle<C, F>: Clone,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let scale_marks = match self.scale {
            Scale::Fixed(interval) => {
                self.range.clone().into_iter().step_by(interval)
            }
            Scale::RangeFraction(fraction) => {
                let len = self.range.len();
                self.range.clone().into_iter().step_by(len / fraction)
            }
        };
        match self.placement {
            Placement::X { x1, x2, y } => {
                Line { start: Point { x: x1, y }, end: Point { x: x2, y } }
                    .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                    .draw(display)?;
                let title = Text::new(self.title, Point { x: x1, y: y + 10 })
                    .into_styled(self.text_style);
                let title = title.translate(Point { x: (x2 - x1) / 2 - title.size().width as i32 / 2, y: 0 });
                title.draw(display)?;

                for mark in scale_marks {
                    let x = mark.scale_between_ranges(&self.range, &(x1..x2));
                    Line { start: Point { x, y: y - self.tick_size as i32 }, end: Point { x, y: y + self.tick_size as i32 } }
                        .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                        .draw(display)?;
                    let mut buf: String::<U8> = String::new();
                    write!(buf, "{}", mark).unwrap();
                    Text::new(&buf, Point { x: x + 2, y: y + 2 }).into_styled(self.text_style).draw(display)?;
                }
            }
            Placement::Y { y1, y2, x } => {
                Line { start: Point { x, y: y1 }, end: Point { x, y: y2 } }
                    .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                    .draw(display)?;

                let mut max_tick_text_width = 0;
                for mark in scale_marks {
                    let y = mark.scale_between_ranges(&self.range, &(y2..y1));
                    Line { start: Point { x: x - self.tick_size as i32, y }, end: Point { x: x + self.tick_size as i32, y } }
                        .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                        .draw(display)?;
                    let mut buf: String::<U8> = String::new();
                    write!(buf, "{}", mark).unwrap();
                    let tick_val = Text::new(&buf, Point { x, y }).into_styled(self.text_style);
                    let tick_val = tick_val.translate(Point { x: -(tick_val.size().width as i32) - 2, y: 2 });
                    if tick_val.size().width > max_tick_text_width { max_tick_text_width = tick_val.size().width }
                    tick_val.draw(display)?;
                }
                let title = Text::new(self.title, Point { x, y: y1 })
                    .into_styled(self.text_style);
                let title = title.translate(Point { x: -(title.size().width as i32) - max_tick_text_width as i32 - self.tick_size as i32 - 2, y: (y2 - y1) / 2 });
                title.draw(display)?;
            }
        }
        Ok(())
    }
}