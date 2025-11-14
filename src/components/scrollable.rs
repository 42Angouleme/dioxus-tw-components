use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ScrollableProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Scrollable(mut props: ScrollableProps) -> Element {
    let default_classes = "scrollable";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}
