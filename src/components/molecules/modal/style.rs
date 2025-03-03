use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for ModalProps {}

// Used to make a "useless" div which does not create a newline that wrap our trigger with our trigger_closure
// Also used by ModalCancelProps
impl Class for ModalTriggerProps {
    fn base(&self) -> &'static str {
        "px-4 py-2 text-sm font-medium text-foreground bg-background border border-input rounded-global-radius shadow-sm shadow-global-shadow whitespace-nowrap cursor-pointer hover:bg-accent hover:text-accent-foreground"
    }
}

impl Class for ModalCloseProps {
    fn base(&self) -> &'static str {
        "absolute top-4 right-4 rounded-global-radius border border-transparent cursor-pointer active:border-border transition-all"
    }
}

impl Class for ModalContentProps {
    fn base(&self) -> &'static str {
        "p-4 flex flex-col top-[50%] left-[50%] z-50 min-w-96 bg-background border border-border rounded-global-radius fixed translate-x-[-50%] translate-y-[-50%] data-[state=inactive]:invisible"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => {
                "data-[state=inactive]:translate-y-full data-[state=inactive]:opacity-0 transition-all duration-300"
            }
        })
    }
}

impl Class for ModalBackgroundProps {
    fn base(&self) -> &'static str {
        "w-full h-full top-0 left-0 z-40 opacity-75 fixed data-[state=inactive]:invisible"
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
