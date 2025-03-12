use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct PlaceholderProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
}

pub fn Placeholder(mut props: PlaceholderProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes }
    }
}
