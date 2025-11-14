use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SelectGroupProps {
    #[props(extends = select, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    default_value: String,

    #[props(optional)]
    value: Signal<String>,

    #[props(optional)]
    onchange: EventHandler<FormEvent>,

    children: Element,
}

#[component]
pub fn SelectGroup(mut props: SelectGroupProps) -> Element {
    let default_classes = "select-group";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let oninput = move |event: FormEvent| {
        props.value.set(event.data.value().clone());
        props.onchange.call(event);
    };

    rsx! {
        select {
            oninput,
            value: props.default_value,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectPlaceholderProps {
    #[props(extends = option, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectPlaceholder(mut props: SelectPlaceholderProps) -> Element {
    let default_classes = "select-placeholder";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        option { disabled: true, selected: true, value: r#"{""}"#, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectLabelProps {
    #[props(extends = optgroup, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SelectLabel(mut props: SelectLabelProps) -> Element {
    let default_classes = "select-label";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        optgroup { ..props.attributes }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectItemProps {
    #[props(extends = option, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default = None)]
    selected: Option<bool>,

    children: Element,
}

#[component]
pub fn SelectItem(mut props: SelectItemProps) -> Element {
    let default_classes = "select-item";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

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
