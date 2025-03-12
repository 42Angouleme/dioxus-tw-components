use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct DocsTemplateProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn DocsTemplate(mut props: DocsTemplateProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

impl Class for DocsTemplateProps {}
