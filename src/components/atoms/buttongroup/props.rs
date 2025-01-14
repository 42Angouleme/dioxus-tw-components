use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ButtonGroupProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for ButtonGroupProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: Ok(VNode::default())
        }
    }
}

pub fn ButtonGroup(mut props: ButtonGroupProps) -> Element {
    props.update_class_attribute();

    rsx!(
        div { ..props.attributes,{props.children} }
    )
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ButtonGroupItemProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for ButtonGroupItemProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: Ok(VNode::default())
        }
    }
}

pub fn ButtonGroupItem(mut props: ButtonGroupItemProps) -> Element {
    props.update_class_attribute();

    let onclick = move |event| props.onclick.call(event);

    rsx!(
        button { onclick, ..props.attributes, {props.children} }
    )
}
