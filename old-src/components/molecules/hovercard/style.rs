use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for HoverCardProps {
    fn base(&self) -> &'static str {
        "group relative text-foreground"
    }
}

impl Class for HoverCardTriggerProps {
    fn base(&self) -> &'static str {
        "text-sm font-medium whitespace-nowrap group-hover:underline transition-all"
    }
}

impl Class for HoverCardContentProps {
    fn base(&self) -> &'static str {
        "p-4 bg-background whitespace-nowrap border border-border rounded-global-radius shadow-global-shadow z-30 opacity-100 absolute mt-2 data-[state=inactive]:invisible"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => {
                "transition-all duration-100 data-[state=inactive]:scale-90 data-[state=active]:scale-100 data-[state=inactive]:opacity-0"
            }
        })
    }
}
