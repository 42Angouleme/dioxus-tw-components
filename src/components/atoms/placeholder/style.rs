use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for PlaceholderProps {
    fn base(&self) -> &'static str {
        "rounded-global-radius w-24 h-24"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Default => "bg-foreground/20",
            Color::Primary => "bg-primary/20",
            Color::Secondary => "bg-secondary/20",
            Color::Destructive => "bg-destructive/20",
            Color::Success => "bg-success/20",
            Color::Accent => "bg-accent/20",
            Color::Muted => "bg-muted/20",
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light => "animate-pulse",
            Animation::Full => {
                "relative overflow-hidden before:absolute before:inset-0 before:-translate-x-full before:bg-linear-to-r before:from-transparent before:via-white/40 before:animate-[shimmer_2s_infinite]"
            }
        })
    }
}
