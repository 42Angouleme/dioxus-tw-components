use super::props::*;
use crate::types::*;

impl BaseClass for ScrollableProps {
    fn base(&self) -> &'static str {
        "px-2 py-4 scrollbar"
    }
}

impl Variation for ScrollableProps {
    fn variant(&self) -> &'static str {
        match self.horizontal {
            // TODO WIP Trying to make horizontal scroll with mouse wheel
            true => "overflow-y-auto overflow-x-hidden -rotate-90 origin-[right_top] -rotate-90",
            false => "overflow-y-auto overflow-x-hidden grid-flow-row",
        }
    }
}

// Check input.css to personalize scrollbar class
// def_class_with_variant!(ScrollableClass, r#"px-2 py-4 scrollbar"#, horizontal: ScrollableHorizontal);

// def_variant!(
//     ScrollableHorizontal,
//     Vertical => r#"overflow-y-auto overflow-x-hidden grid-flow-row"#,
//     Horizontal => r#"overflow-y-auto overflow-x-hidden -rotate-90 origin-[right_top] -rotate-90"#
// );

// impl From<bool> for ScrollableHorizontal {
//     fn from(horizontal: bool) -> Self {
//         match horizontal {
//             true => ScrollableHorizontal::Horizontal,
//             false => ScrollableHorizontal::Vertical,
//         }
//     }
// }
