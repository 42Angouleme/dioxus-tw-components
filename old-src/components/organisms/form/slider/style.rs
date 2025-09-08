use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for SliderProps {
    fn base(&self) -> &'static str {
        "w-full disabled:cursor-not-allowed disabled:opacity-50 cursor-pointer"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Default => "accent-foreground",
            Color::Primary => "accent-primary",
            Color::Secondary => "accent-secondary",
            Color::Destructive => "accent-destructive",
            Color::Success => "accent-success",
            Color::Accent => "accent-accent",
            Color::Muted => "accent-muted",
        })
    }
}

impl Class for SliderLabelProps {
    fn base(&self) -> &'static str {
        "text-xs"
    }
}

impl Class for SliderTicksProps {}
