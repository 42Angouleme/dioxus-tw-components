use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for AccordionProps {
    fn base(&self) -> &'static str {
        "space-y-2"
    }
}

impl Class for AccordionItemProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl Class for AccordionTriggerProps {
    fn base(&self) -> &'static str {
        "flex items-center justify-between w-full font-medium text-foreground group"
    }
}

impl Class for AccordionContentProps {
    fn base(&self) -> &'static str {
        "text-sm text-foreground overflow-hidden mb-2"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transition-all",
        })
    }
}
