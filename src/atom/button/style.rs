use super::props::*;
use crate::types::*;

impl BaseClass for ButtonProps {
    fn base(&self) -> &'static str {
        "text-center font-medium rounded-global-radius shadow-global-shadow transition-colors duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Ghost,
}

// TO CLEAN
impl Variation for ButtonProps {
    fn variant(&self) -> &'static str {
        match self.variant {
            ButtonVariant::Default => "",
            ButtonVariant::Outline => match self.color {
                Color::Default => {
                    "border bg-transparentborder-foreground text-foreground hover:bg-foreground/40"
                }
                Color::Primary => "border bg-transparentborder-primary text-primary hover:bg-primary/90 hover:text-primary-foreground",
                Color::Secondary => "border bg-transparentborder-secondary text-secondary hover:bg-secondary/90 hover:text-secondary-foreground",
                Color::Destructive => "border bg-transparentborder-destructive text-destructive hover:bg-destructive/90 hover:text-destructive-foreground",
                Color::Success => "border bg-transparentborder-success text-success hover:bg-success/90 hover:text-success-foreground",
                Color::Accent => "border bg-transparentborder-accent text-accent hover:bg-accent/90 hover:text-accent-foreground",
                Color::Muted => "border bg-transparentborder-muted text-muted hover:bg-muted/90 hover:text-muted-foreground",
            },
            ButtonVariant::Ghost => match self.color {
                Color::Default => {
                    "bg-transparentborder-foreground text-foreground hover:bg-foreground/40"
                }
                Color::Primary => "bg-transparentborder-primary text-primary hover:bg-primary/90 hover:text-primary-foreground",
                Color::Secondary => "bg-transparentborder-secondary text-secondary hover:bg-secondary/90 hover:text-secondary-foreground",
                Color::Destructive => "bg-transparentborder-destructive text-destructive hover:bg-destructive/90 hover:text-destructive-foreground",
                Color::Success => "bg-transparentborder-success text-success hover:bg-success/90 hover:text-success-foreground",
                Color::Accent => "bg-transparentborder-accent text-accent hover:bg-accent/90 hover:text-accent-foreground",
                Color::Muted => "bg-transparentborder-muted text-muted hover:bg-muted/90 hover:text-muted-foreground",
            },
        }
    }
}

impl Colorable for ButtonProps {
    fn color(&self) -> &'static str {
        match self.color {
            Color::Default => "bg-background text-foreground hover:bg-foreground/20 active:bg-foreground/30 active:shadow",
            Color::Primary => "bg-primary text-primary-foreground border-primary hover:bg-primary/90 active:bg-primary/80 active:shadow",
            Color::Secondary => "bg-secondary text-secondary-foreground border-secondary hover:bg-secondary/90 active:bg-secondary/80 active:shadow",
            Color::Destructive => "bg-destructive text-destructive-foreground border-destructive hover:bg-destructive/90 active:bg-destructive/80 active:shadow",
            Color::Success => "bg-success text-success-foreground border-success hover:bg-success/90 active:bg-success/80 active:shadow",
            Color::Accent => "bg-accent text-accent-foreground border-accent hover:bg-accent/90 active:bg-accent/80 active:shadow",
            Color::Muted => "bg-muted text-muted-foreground border-muted hover:bg-muted/90 active:bg-muted/80 active:shadow",
        }
    }
}

impl Sizable for ButtonProps {
    fn size(&self) -> &'static str {
        match self.size {
            Size::Xs => "px-2.5 py-1.5 text-xs",
            Size::Sm => "px-3 py-2 text-sm",
            Size::Md => "px-5 py-[9px] text-base",
            Size::Lg => "px-6 py-3 text-lg",
            Size::Xl => "px-8 py-4 text-xl",
        }
    }
}
