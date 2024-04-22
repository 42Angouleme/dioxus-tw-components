use super::props::*;
use crate::attributes::*;

impl BaseClass for TabsProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl BaseClass for TabsListProps {
    fn base(&self) -> &'static str {
        "w-full flex h-8 p-1 items-center justify-center rounded-global-radius bg-muted text-muted-foreground"
    }
}

impl BaseClass for TabsTriggerProps {
    fn base(&self) -> &'static str {
        "flex grow items-center justify-center whitespace-nowrap rounded-global-radius px-2 py-0.5 text-sm font-semibold ring-offset-background transition-all duration-75
        data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow"
    }
}

impl BaseClass for TabsContentProps {
    fn base(&self) -> &'static str {
        "mt-2 border border-border rounded-global-radius shadow"
    }
}
