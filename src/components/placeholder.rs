use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PlaceholderProps {
    /// Additional attributes to apply to the element
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Placeholder(props: PlaceholderProps) -> Element {
    rsx! {
        div { class: "placeholder", ..props.attributes }
    }
}
