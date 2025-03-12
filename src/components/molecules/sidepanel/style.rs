use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for SidePanelProps {}

impl Class for SidePanelTriggerProps {
    fn base(&self) -> &'static str {
        "px-4 py-2 text-sm font-medium text-foreground bg-background border border-input rounded-global-radius whitespace-nowrap cursor-pointer hover:bg-accent hover:text-accent-foreground"
    }
}

impl Class for SidePanelCloseProps {
    fn base(&self) -> &'static str {
        "absolute top-4 right-4 rounded-global-radius border border-transparent cursor-pointer active:border-border transition-all"
    }
}

impl Class for SidePanelContentProps {
    fn base(&self) -> &'static str {
        match *self.side.read() {
            Side::Top => {
                "fixed z-800 p-4 overflow-hidden flex flex-col bottom-[100%] left-[50%]   w-96 bg-background border border-border rounded-global-radius translate-x-[-50%]  translate-y-[100%]  data-[state=inactive]:invisible"
            }
            Side::Bottom => {
                "fixed z-800 p-4 overflow-hidden flex flex-col top-[100%]    left-[50%]   w-96 bg-background border border-border rounded-global-radius translate-x-[-50%]  translate-y-[-100%] data-[state=inactive]:invisible"
            }
            Side::Left => {
                "fixed z-800 p-4 overflow-hidden flex flex-col top-[50%]     right-[100%] w-96 bg-background border border-border rounded-global-radius translate-x-[100%]  translate-y-[-50%]  data-[state=inactive]:invisible"
            }
            Side::Right => {
                "fixed z-800 p-4 overflow-hidden flex flex-col top-[50%]     left-[100%]  w-96 bg-background border border-border rounded-global-radius translate-x-[-100%] translate-y-[-50%]  data-[state=inactive]:invisible"
            }
        }
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => {
                "data-[state=inactive]:opacity-0 transition-all duration-300"
            }
        })
    }

    fn side(&self) -> Option<&'static str> {
        Some(match *self.side.read() {
            Side::Top | Side::Bottom => "data-[state=inactive]:translate-y-[0%]",
            Side::Left | Side::Right => "data-[state=inactive]:translate-x-[0%]",
        })
    }
}

impl Class for SidePanelBackgroundProps {
    fn base(&self) -> &'static str {
        "fixed overflow-hidden w-screen h-screen top-0 left-0 z-799 opacity-15 data-[state=inactive]:invisible"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Primary => "bg-primary",
            Color::Secondary => "bg-secondary",
            Color::Destructive => "bg-destructive",
            Color::Success => "bg-success",
            _ => "bg-foreground",
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => {
                "data-[state=inactive]:opacity-0 data-[state=inactive]:invisible transition-all duration-300"
            }
        })
    }
}
