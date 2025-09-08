use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for InputProps {
    fn base(&self) -> &'static str {
        "peer flex w-full px-2 py-1.5 text-foreground bg-background border rounded-global-radius hover:brightness-105 focus:outline-hidden focus:brightness-105 focus:ring-ring focus:ring-offset-2 focus:ring-2 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:brightness-100 file:font-medium file:bg-input file:rounded-xs file:border-0 file:items-center file:justify-center"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Primary => "border-primary",
            Color::Secondary => "border-secondary",
            Color::Destructive => "border-destructive",
            Color::Success => "border-success",
            _ => "border-input",
        })
    }

    fn size(&self) -> Option<&'static str> {
        Some(match *self.size.read() {
            Size::Xs => "h-4 text-xs",
            Size::Sm => "h-6 text-xs",
            Size::Md => "h-9 text-sm",
            Size::Lg => "h-11 text-base",
            Size::Xl => "h-14 text-lg",
        })
    }
}
