use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SelectGroupProps {
    #[props(extends = select, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    children: Element,
}

impl std::default::Default for SelectGroupProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            oninput: EventHandler::<FormEvent>::default(),
            children: rsx! {},
        }
    }
}

pub fn SelectGroup(mut props: SelectGroupProps) -> Element {
    props.update_class_attribute();

    let oninput = move |event| props.oninput.call(event);

    rsx! {
        select { oninput, ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SelectPlaceholderProps {
    #[props(extends = option, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for SelectPlaceholderProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

pub fn SelectPlaceholder(mut props: SelectPlaceholderProps) -> Element {
    props.update_class_attribute();

    rsx! {
        option { disabled: true, selected: true, value: r#"{""}"#, {props.children} }
    }
}

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct SelectLabelProps {
    #[props(extends = optgroup, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

pub fn SelectLabel(mut props: SelectLabelProps) -> Element {
    props.update_class_attribute();

    rsx! {
        optgroup { ..props.attributes }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SelectItemProps {
    #[props(extends = option, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default = None)]
    selected: Option<bool>,

    children: Element,
}

impl std::default::Default for SelectItemProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            selected: None,
            children: rsx! {},
        }
    }
}

pub fn SelectItem(mut props: SelectItemProps) -> Element {
    props.update_class_attribute();

    if let Some(selected) = props.selected {
        rsx! {
            option { selected, ..props.attributes, {props.children} }
        }
    } else {
        rsx! {
            option { ..props.attributes,{props.children} }
        }
    }
}
