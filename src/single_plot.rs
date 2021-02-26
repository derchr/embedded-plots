use crate::curve::Curve;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::pixelcolor::PixelColor;
use crate::axis::{Scale, Placement, Axis};
use embedded_graphics::style::TextStyleBuilder;
use embedded_graphics::fonts::Font6x8;

pub struct SinglePlot<'a> {
    curve: &'a Curve<'a>,
    x_scale: Scale,
    y_scale: Scale,
}

impl<'a> SinglePlot<'a> {
    pub fn new(curve: &'a Curve<'a>, x_scale: Scale, y_scale: Scale) -> SinglePlot {
        SinglePlot { curve, x_scale, y_scale }
    }

    pub fn into_drawable<C: PixelColor + Default>(self, top_left: Point, bottom_right: Point) -> DrawableSinglePlot<'a, C> {
        DrawableSinglePlot { plot: self, color: None, text_color: None, axis_color: None, thickness: None, axis_thickness: None, top_left, bottom_right }
    }
}

pub struct DrawableSinglePlot<'a, C>
    where
        C: PixelColor + Default,
{
    plot: SinglePlot<'a>,
    color: Option<C>,
    text_color: Option<C>,
    axis_color: Option<C>,
    thickness: Option<usize>,
    axis_thickness: Option<usize>,
    top_left: Point,
    bottom_right: Point,
}

impl<'a, C> DrawableSinglePlot<'a, C>
    where
        C: PixelColor + Default,
{
    pub fn set_color(mut self, color: C) -> DrawableSinglePlot<'a, C> {
        self.color = Some(color);
        self
    }

    pub fn set_text_color(mut self, color: C) -> DrawableSinglePlot<'a, C> {
        self.text_color = Some(color);
        self
    }

    pub fn set_axis_color(mut self, color: C) -> DrawableSinglePlot<'a, C> {
        self.axis_color = Some(color);
        self
    }

    pub fn set_thickness(mut self, thickness: usize) -> DrawableSinglePlot<'a, C> {
        self.thickness = Some(thickness);
        self
    }

    pub fn set_axis_thickness(mut self, thickness: usize) -> DrawableSinglePlot<'a, C> {
        self.axis_thickness = Some(thickness);
        self
    }

}

impl<'a, C> Drawable<C> for DrawableSinglePlot<'a, C>
    where
        C: PixelColor + Default,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let color = match self.color {
            None => C::default(),
            Some(c) => c,
        };
        let text_color = match self.text_color {
            None => color,
            Some(c) => c,
        };
        let axis_color = match self.axis_color {
            None => color,
            Some(c) => c,
        };

        let thickness = match self.thickness {
            None => 2,
            Some(t) => t,
        };

        let axis_thickness = match self.axis_thickness {
            None => thickness,
            Some(t) => t,
        };

        let text_style = TextStyleBuilder::new(Font6x8)
            .text_color(text_color)
            .build();

        Axis::new("X", self.plot.curve.x_range.clone(), self.plot.x_scale)
            .into_drawable_axis(Placement::X { x1: self.top_left.x, x2: self.bottom_right.x, y: self.bottom_right.y }, axis_color, text_style, 2, axis_thickness)
            .draw(display)?;

        Axis::new("Y", self.plot.curve.y_range.clone(), self.plot.y_scale)
            .into_drawable_axis(Placement::Y { y1: self.top_left.y, y2: self.bottom_right.y, x: self.top_left.x }, axis_color, text_style, 2,axis_thickness)
            .draw(display)?;

        self.plot.curve.into_drawable_curve(
            &self.top_left,
            &self.bottom_right,
        ).set_color(color)
            .set_thickness(thickness)
            .draw(display)?;
        Ok(())
    }
}