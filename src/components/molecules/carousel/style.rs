use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for CarouselProps {
    fn base(&self) -> &'static str {
        "container flex items-center"
    }
}

impl Class for CarouselWindowProps {
    fn base(&self) -> &'static str {
        "relative overflow-hidden grow rounded-global-radius border border-border"
    }
}

impl Class for CarouselContentProps {
    fn base(&self) -> &'static str {
        "flex"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transform transition-transform duration-500",
        })
    }
}

impl Class for CarouselItemProps {
    fn base(&self) -> &'static str {
        "relative min-w-0 shrink-0 grow-0 basis-full p-2"
    }
}

impl Class for CarouselTriggerProps {
    fn base(&self) -> &'static str {
        "size-10 flex flex-nowrap items-center justify-center cursor-pointer"
    }
}
