use crate::{web_sys::HtmlParagraphElement , *};
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Paragraph<EmptyFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<EmptyFlag>,
}

impl Paragraph<EmptyFlagSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("p")
                .attr("class", "paragraph"),
            flags: PhantomData,
        }
    }
}

impl Element for Paragraph<EmptyFlagNotSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlag> UpdateRawEl<RawHtmlEl> for Paragraph<EmptyFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<EmptyFlag> Styleable<'_, RawHtmlEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> KeyboardEventAware<RawHtmlEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> Hoverable<RawHtmlEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> MutableViewport<RawHtmlEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> Hookable<RawHtmlEl> for Paragraph<EmptyFlag> {
    type WSElement = HtmlParagraphElement;
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag> Paragraph<EmptyFlag> {
    pub fn content(mut self, content: impl IntoOptionElement<'a> + 'a) -> Paragraph<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child(content);
        self.into_type()
    }

    pub fn content_signal(
        mut self,
        content: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Paragraph<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child_signal(content);
        self.into_type()
    }

    pub fn contents(
        mut self,
        contents: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Paragraph<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children(contents);
        self.into_type()
    }

    pub fn contents_signal_vec(
        mut self,
        contents: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Paragraph<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children_signal_vec(contents);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Paragraph<NewEmptyFlag> {
        Paragraph {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}