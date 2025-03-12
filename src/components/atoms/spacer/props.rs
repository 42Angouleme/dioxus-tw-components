use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct SpacerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

pub fn Spacer(mut props: SpacerProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes }
    }
}
