use super::props::*;
use crate::attributes::*;

impl Class for SelectGroupProps {
    fn base(&self) -> &'static str {
        "flex w-full px-2 py-1.5 h-9 bg-background border border-input rounded-global-radius text-foreground text-sm cursor-pointer"
    }
}

impl Class for SelectPlaceholderProps {
    fn base(&self) -> &'static str {
        "text-foreground hidden font-bold"
    }
}

impl Class for SelectLabelProps {
    fn base(&self) -> &'static str {
        "text-foreground text-sm font-semibold"
    }
}

impl Class for SelectItemProps {
    fn base(&self) -> &'static str {
        "text-foreground text-sm"
    }
}
