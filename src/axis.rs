use core::ops::Range;
use embedded_graphics::prelude::*;
use embedded_graphics::style::{TextStyle};

use crate::drawable_axis::DrawableAxis;

pub enum Placement {
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

pub struct Axis<'a> {
    title: &'a str,
    range: Range<i32>,
    scale: Scale,
}

impl<'a> Axis<'a>
{
    pub fn new(title: &'a str, range: Range<i32>, scale: Scale) -> Axis<'a> {
        Axis{title, range, scale}
    }

    pub fn into_drawable_axis<C, F>(self, placement: Placement, plot_color: C, text_style: TextStyle<C, F>, tick_height: usize, thickness: usize) -> DrawableAxis<'a, C, F>
        where
            C: PixelColor,
            F: Font,
            TextStyle<C, F>: Clone,
    {
        DrawableAxis::new(self.title,placement,self.range,self.scale,plot_color,text_style,tick_height,thickness)
    }
}
