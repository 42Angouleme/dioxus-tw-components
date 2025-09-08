use super::props::*;
use crate::attributes::*;

impl Class for TableProps {
    fn base(&self) -> &'static str {
        "w-full caption-bottom text-sm text-foreground bg-background"
    }
}

impl Class for TableHeaderProps {
    fn base(&self) -> &'static str {
        "[&_tr]:border-b"
    }
}

impl Class for TableBodyProps {
    fn base(&self) -> &'static str {
        "[&_tr:last-child]:border-0"
    }
}

impl Class for TableFooterProps {
    fn base(&self) -> &'static str {
        "border-t bg-muted/50 font-medium last:[&>tr]:border-b-0 hover:bg-muted/70"
    }
}

impl Class for TableHeadProps {
    fn base(&self) -> &'static str {
        "h-10 px-2 text-left align-middle font-medium text-muted-foreground"
    }
}

impl Class for TableRowProps {
    fn base(&self) -> &'static str {
        "border-b transition-colors hover:bg-muted/50"
    }
}

impl Class for TableCellProps {
    fn base(&self) -> &'static str {
        "p-2 align-middle"
    }
}

impl Class for TableCaptionProps {
    fn base(&self) -> &'static str {
        "mt-4 text-sm text-muted-foreground"
    }
}
