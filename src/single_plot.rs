use crate::axis::{Axis, Placement, Scale};
use crate::curve::Curve;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::PixelColor, prelude::Point, Drawable,
};

/// Display agnostic single curve plot object
#[derive(Clone, Copy)]
pub struct SinglePlot<'a> {
    /// curve to be drawn on the plot
    curve: &'a Curve<'a>,
    /// range of X axis on which curve will be drawn
    x_scale: Scale,
    /// range of Y axis on which curve will be drawn
    y_scale: Scale,
}

impl<'a> SinglePlot<'a> {
    /// create SinglePlot object with manual range
    pub fn new(curve: &'a Curve<'a>, x_scale: Scale, y_scale: Scale) -> SinglePlot {
        SinglePlot {
            curve,
            x_scale,
            y_scale,
        }
    }
    //TODO: add auto range plot constructor
    /// convert to drawable form for specific display
    pub fn into_drawable<C: PixelColor + Default>(
        self,
        top_left: Point,
        bottom_right: Point,
    ) -> DrawableSinglePlot<'a, C> {
        DrawableSinglePlot {
            plot: self,
            color: None,
            text_color: None,
            axis_color: None,
            thickness: None,
            axis_thickness: None,
            top_left,
            bottom_right,
        }
    }
}

/// Drawable single plot object, constructed for specific display
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

/// builder methods to modify plot decoration
impl<'a, C> DrawableSinglePlot<'a, C>
where
    C: PixelColor + Default,
{
    pub fn set_color(mut self, color: C) -> DrawableSinglePlot<'a, C> {
        self.color = Some(color);
        self
    }

    /// if not set, main color will be used
    pub fn set_text_color(mut self, color: C) -> DrawableSinglePlot<'a, C> {
        self.text_color = Some(color);
        self
    }

    /// if not set, main color will be used
    pub fn set_axis_color(mut self, color: C) -> DrawableSinglePlot<'a, C> {
        self.axis_color = Some(color);
        self
    }

    /// set curve thickness
    pub fn set_thickness(mut self, thickness: usize) -> DrawableSinglePlot<'a, C> {
        self.thickness = Some(thickness);
        self
    }

    ///set axis thickness
    pub fn set_axis_thickness(mut self, thickness: usize) -> DrawableSinglePlot<'a, C> {
        self.axis_thickness = Some(thickness);
        self
    }
    
    //TODO: add axis ticks thickness
}
impl<'a, C> Drawable for DrawableSinglePlot<'a, C>
where
    C: PixelColor + Default,
{
    type Color = C;
    type Output = ();

    /// most important function - draw the plot on the display
    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let color = self.color.unwrap_or_default();
        let text_color = self.text_color.unwrap_or(color);
        let axis_color = self.axis_color.unwrap_or(color);
        let thickness = self.thickness.unwrap_or(2);
        let axis_thickness = self.axis_thickness.unwrap_or(thickness);
        let text_style = MonoTextStyleBuilder::new().text_color(text_color).build();

        Axis::new(self.plot.curve.x_range.clone())
            .set_title("X")
            .set_scale(self.plot.x_scale)
            .into_drawable_axis(Placement::X {
                x1: self.top_left.x,
                x2: self.bottom_right.x,
                y: self.bottom_right.y,
            })
            .set_color(axis_color)
            .set_text_style(text_style)
            .set_tick_size(2)
            .set_thickness(axis_thickness)
            .draw(display)?;

        Axis::new(self.plot.curve.y_range.clone())
            .set_title("Y")
            .set_scale(self.plot.y_scale)
            .into_drawable_axis(Placement::Y {
                y1: self.top_left.y,
                y2: self.bottom_right.y,
                x: self.top_left.x,
            })
            .set_color(axis_color)
            .set_text_style(text_style)
            .set_tick_size(2)
            .set_thickness(axis_thickness)
            .draw(display)?;

        self.plot
            .curve
            .into_drawable_curve(&self.top_left, &self.bottom_right)
            .set_color(color)
            .set_thickness(thickness)
            .draw(display)?;

        Ok(())
    }
}
