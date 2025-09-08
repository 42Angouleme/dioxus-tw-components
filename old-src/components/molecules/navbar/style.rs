use super::props::*;
use crate::attributes::*;

impl Class for NavbarProps {
    fn base(&self) -> &'static str {
        "container flex h-14 max-w-(--breakpoint-2xl) items-center whitespace-nowrap"
    }
}
