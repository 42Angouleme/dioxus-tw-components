use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ButtonGroupProps {
    /// Additional attributes to apply to the element
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children element
    children: Element,
}

#[component]
pub fn ButtonGroup(mut props: ButtonGroupProps) -> Element {
    let default_classes = "buttongroup";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}
