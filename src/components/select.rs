use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps {
    #[props(extends = select, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    children: Element,
}

#[component]
pub fn Select(mut props: SelectProps) -> Element {
    let default_classes = "select";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let oninput = move |event| props.oninput.call(event);

    rsx! {
        select { oninput, ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectItemProps {
    #[props(extends = option, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    value: String,

    children: Element,
}

#[component]
pub fn SelectItem(mut props: SelectItemProps) -> Element {
    let default_classes = "select-item";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        option { 
            value: props.value,
            ..props.attributes, 
            {props.children} 
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectTrigger(mut props: SelectTriggerProps) -> Element {
    let default_classes = "select-trigger";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectContent(mut props: SelectContentProps) -> Element {
    let default_classes = "select-content";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}
