use super::props::*;
use crate::attributes::*;

impl Class for MarkdownProps {
    fn base(&self) -> &'static str {
        "prose dark:prose-invert max-w-none"
    }
}
