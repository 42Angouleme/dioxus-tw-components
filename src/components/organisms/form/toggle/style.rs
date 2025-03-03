use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for ToggleProps {
    fn base(&self) -> &'static str {
        "peer relative bg-input rounded-full focus:outline-hidden focus:ring-2 focus:ring-black focus:ring-offset-2 data-[state=active]:after:translate-x-full data-[state=active]:after:border-white after:content-[''] after:absolute after:bg-background after:border-input after:border after:rounded-full disabled:opacity-40"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Default => "data-[state=active]:bg-foreground",
            Color::Primary => "data-[state=active]:bg-primary",
            Color::Secondary => "data-[state=active]:bg-secondary",
            Color::Destructive => "data-[state=active]:bg-destructive",
            Color::Success => "data-[state=active]:bg-success",
            _ => "",
        })
    }

    fn size(&self) -> Option<&'static str> {
        Some(match *self.size.read() {
            Size::Lg | Size::Xl => "w-14 h-7 after:top-0.5 after:start-[4px] after:h-6 after:w-6",
            Size::Md => "w-11 h-6 after:top-[2px] after:start-[2px] after:h-5 after:w-5",
            Size::Sm | Size::Xs => "w-9 h-5 after:top-[2px] after:start-[2px] after:h-4 after:w-4",
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light => "after:transition-all",
            Animation::Full => "after:transition-all transition-colors duration-200",
        })
    }
}
