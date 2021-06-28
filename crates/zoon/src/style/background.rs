use crate::*;
use crate::style::{StaticCSSProps, DynamicCSSProps, box_css_signal};

#[derive(Default)]
pub struct Background<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl Background<'_> {
    pub fn color_signal(mut self, color: impl Signal<Item = impl Color<'static> + 'static> + Unpin + 'static) -> Self {
        self.dynamic_css_props.insert("background-color", box_css_signal(color));
        self
    }
}

impl<'a> Style<'a> for Background<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}