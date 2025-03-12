use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for ProgressBarProps {
    fn base(&self) -> &'static str {
        "w-full rounded-global-radius"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Primary => "bg-primary",
            Color::Secondary => "bg-secondary",
            Color::Destructive => "bg-destructive",
            Color::Success => "bg-success",
            Color::Muted => "bg-muted",
            Color::Accent => "bg-accent",
            _ => "bg-background",
        })
    }

    fn size(&self) -> Option<&'static str> {
        Some(match *self.size.read() {
            Size::Xs => "h-1 text-xs",
            Size::Sm => "h-2 text-xs",
            Size::Md => "h-4 text-sm",
            Size::Lg => "h-6 text-base",
            Size::Xl => "h-8 text-lg",
        })
    }
}

impl Class for ProgressBarInnerProps {
    fn base(&self) -> &'static str {
        "h-full rounded-global-radius flex items-center justify-center transition-all"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Primary => "bg-primary *:text-primary-foreground",
            Color::Secondary => "bg-secondary *:text-secondary-foreground",
            Color::Destructive => "bg-destructive *:text-destructive-foreground",
            Color::Success => "bg-success *:text-success-foreground",
            _ => "bg-foreground *:text-background",
        })
    }
}

impl Class for ProgressLabelProps {}
