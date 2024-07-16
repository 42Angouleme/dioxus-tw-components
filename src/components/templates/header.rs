use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct HeaderTemplateProps {
    #[props(extends = header, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn HeaderTemplate(mut props: HeaderTemplateProps) -> Element {
    props.update_class_attribute();

    rsx!(
        header { ..props.attributes, { props.children } }
    )
}

impl Class for HeaderTemplateProps {
    fn base(&self) -> &'static str {
        "sticky top-0 bg-background/65 backdrop-blur border-b border-border"
    }
}
