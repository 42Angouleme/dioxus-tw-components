use super::props::*;
use crate::attributes::*;

impl Class for SortTableProps {
    fn base(&self) -> &'static str {
        "w-full caption-bottom text-sm text-foreground bg-background"
    }
}
