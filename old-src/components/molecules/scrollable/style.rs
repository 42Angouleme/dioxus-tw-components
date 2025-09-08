use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for ScrollableProps {
    fn base(&self) -> &'static str {
        "scrollbar p-2 border border-border rounded-global-radius min-w-32"
    }

    fn orientation(&self) -> Option<&'static str> {
        Some(match *self.orientation.read() {
            Orientation::Horizontal => {
                "overflow-y-auto overflow-x-hidden -rotate-90 origin-[right_top] -rotate-90"
            }
            Orientation::Vertical => "overflow-y-auto overflow-x-hidden grid-flow-row",
        })
    }
}
