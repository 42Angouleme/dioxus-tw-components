use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for CheckboxProps {
    fn base(&self) -> &'static str {
        "peer"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Primary => "accent-primary focus:ring-primary focus:ring-2 focus:ring-offset-1",
            Color::Secondary => {
                "accent-secondary focus:ring-secondary focus:ring-2 focus:ring-offset-1"
            }
            Color::Destructive => {
                "accent-destructive focus:ring-destructive focus:ring-2 focus:ring-offset-1"
            }
            Color::Success => "accent-success focus:ring-success focus:ring-2 focus:ring-offset-1",
            _ => "accent-foreground focus:ring-foreground focus:ring-2 focus:ring-offset-1",
        })
    }

    fn size(&self) -> Option<&'static str> {
        Some(match *self.size.read() {
            Size::Xs => "size-2",
            Size::Sm => "size-3",
            Size::Md => "",
            Size::Lg => "size-5",
            Size::Xl => "size-8",
        })
    }
}
