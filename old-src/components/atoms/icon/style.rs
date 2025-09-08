use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for IconProps {
    fn base(&self) -> &'static str {
        "select-none"
    }

    fn size(&self) -> Option<&'static str> {
        Some(match *self.size.read() {
            Size::Xs => "text-base",
            Size::Sm => "text-xl",
            Size::Md => "text-2xl",
            Size::Lg => "text-4xl",
            Size::Xl => "text-6xl",
        })
    }
}
