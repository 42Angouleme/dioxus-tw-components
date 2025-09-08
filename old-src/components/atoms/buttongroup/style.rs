use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for ButtonGroupProps {
    fn base(&self) -> &'static str {
        "inline-flex border border-border bg-background rounded-global-radius divide-border divide-x"
    }

    fn color(&self) -> Option<&'static str> {
        Some("")
    }

    fn size(&self) -> Option<&'static str> {
        Some("")
    }

    fn animation(&self) -> Option<&'static str> {
        Some("")
    }
}

impl Class for ButtonGroupItemProps {
    fn base(&self) -> &'static str {
        "text-center font-medium first:rounded-l-global-radius last:rounded-r-global-radius disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer select-none"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Default => {
                "bg-foreground text-background hover:bg-foreground/80 active:bg-foreground/70 active:shadow-sm"
            }
            Color::Primary => {
                "bg-primary text-primary-foreground border-primary hover:bg-primary/90 active:bg-primary/80 active:shadow-sm"
            }
            Color::Secondary => {
                "bg-secondary text-secondary-foreground border-secondary hover:bg-secondary/90 active:bg-secondary/80 active:shadow-sm"
            }
            Color::Destructive => {
                "bg-destructive text-destructive-foreground border-destructive hover:bg-destructive/90 active:bg-destructive/80 active:shadow-sm"
            }
            Color::Success => {
                "bg-success text-success-foreground border-success hover:bg-success/90 active:bg-success/80 active:shadow-sm"
            }
            Color::Accent => {
                "bg-accent text-accent-foreground border-accent hover:bg-accent/90 active:bg-accent/80 active:shadow-sm"
            }
            Color::Muted => {
                "bg-muted text-muted-foreground border-muted hover:bg-muted/90 active:bg-muted/80 active:shadow-sm"
            }
        })
    }

    fn size(&self) -> Option<&'static str> {
        Some(match *self.size.read() {
            Size::Xs => "h-4 px-2 text-xs",
            Size::Sm => "h-7 px-3 py-1 text-sm",
            Size::Md => "h-9 px-4 py-2 text-sm",
            Size::Lg => "h-11 px-7 py-2 text-lg",
            Size::Xl => "h-14 px-9 py-3 text-xl",
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => "transition-colors duration-150",
            // Animation::Full => "relative z-30 after:-z-20 after:absolute after:h-1 after:w-1 after:bg-background/80 after:left-5 overflow-hidden after:bottom-0 after:translate-y-full after:rounded-md hover:after:scale-300 hover:after:transition-all hover:after:duration-700 after:transition-all after:duration-700 transition-all duration-700",
        })
    }
}
