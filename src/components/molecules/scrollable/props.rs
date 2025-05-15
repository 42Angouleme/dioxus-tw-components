use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ScrollableProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub orientation: ReadOnlySignal<Orientation>,

    children: Element,
}

impl std::default::Default for ScrollableProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            orientation: ReadOnlySignal::<Orientation>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn Scrollable(mut props: ScrollableProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}
