use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SeparatorProps {
    /// Additional attributes to apply to the element
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Separator(mut props: SeparatorProps) -> Element {
    let default_classes = "separator";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes }
    }
}
