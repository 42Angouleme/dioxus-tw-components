use super::props::*;
use crate::attributes::*;

impl Class for ToasterProps {
    fn base(&self) -> &'static str {
        "fixed z-50 w-full md:max-w-[400px] bottom-0 right-0"
    }
}

impl Class for Toast {
    fn base(&self) -> &'static str {
        "relative group bg-background text-foreground text-sm border border-border shadow-global-shadow p-4 m-2 rounded-global-radius"
    }

    // Color is not a ReadOnlySignal so no need to read()
    fn color(&self) -> Option<&'static str> {
        Some(match self.color {
            Color::Primary => "bg-primary text-primary-foreground",
            Color::Secondary => "bg-secondary text-secondary-foreground",
            Color::Destructive => "bg-destructive text-destructive-foreground",
            Color::Success => "bg-success text-success-foreground",
            _ => "bg-background text-foreground",
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::Light | Animation::Full => {
                "transition-all duration-150 data-[state=opening]:translate-y-full data-[state=open]:translate-y-0 data-[state=closing]:translate-x-full"
            }
            _ => "",
        })
    }
}
