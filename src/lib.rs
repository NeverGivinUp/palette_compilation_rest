use web_sys::{SvgTextElement, Text};
use zoon::{RawEl, RawSvgEl, RawText};
#[derive(Debug, Clone)]
pub struct MeasuredTypeBlock {
    pub lines: Vec<String>,
    /// the distance of the base-line from the top
    pub base_line_start: f64,
    pub leading: f64,
    pub width: f64,
    pub height: f64,
}
pub trait AsElement {
    type DomType: RawEl;
    type Param;
    fn as_element<'a>(
        &self,
        param: Self::Param,
        id: impl Into<Option<&'a str>>,
        class: impl Into<Option<&'a str>>,
    ) -> Self::DomType;
}
use palette::{IntoColor, Oklaba, Srgba};



pub fn oklaba_to_css(color: Oklaba) -> String {
    let rgb: Srgba = color.into_color();
    format!(
        "rgb({} {} {} / {})",
        255.0 * rgb.red,
        255.0 * rgb.green,
        255.0 * rgb.blue,
        rgb.alpha
    )
}

use std::ops::Div;

impl AsElement for MeasuredTypeBlock {
    type DomType = RawSvgEl<SvgTextElement>;
    type Param = ();

    fn as_element<'a>(
        &self,
        _param: (),
        id: impl Into<Option<&'a str>>,
        class: impl Into<Option<&'a str>>,
    ) -> Self::DomType {
        let mut element =
            RawSvgEl::new("text")
                .dom_element_type()
                .children(self.lines.iter().enumerate().map(|(line_num, line)| {
                    let prev_line_relative_horizontal_shift = if line_num == 0 {
                        self.base_line_start
                    } else {
                        self.leading
                    };

                    RawSvgEl::new("tspan")
                        .attr("x", "0")
                        .attr("dy", &format!("{}px", prev_line_relative_horizontal_shift))
                        //.attr("dominant-baseline", "alphabetic")
                        .child(RawText::new(&line))
                }));

        // center the text-anchor in the text.
        //let centering_height_shift = <f64 as Div>::div(self.height, 2.0);
        let centering_height_shift = self.height/ 2.0;
        element = element
            // We cannot use the x-attribute which already is set in the tspans.
            // Since all we want to do is center the text-anchor in the text we can use the
            // text-anchor-attribute.
            .attr("text-anchor", "middle")
            .attr("y", &(-centering_height_shift).to_string());

        if let Some(class) = class.into() {
            element = element.class(class);
        }
        if let Some(id) = id.into() {
            element = element.attr("id", id);
        }
        element
    }
}
