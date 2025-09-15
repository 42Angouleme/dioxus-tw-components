use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SelectGroupProps {
    #[props(extends = select, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    children: Element,
}

#[component]
pub fn SelectGroup(mut props: SelectGroupProps) -> Element {
    let default_classes = "select-group";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let oninput = move |event| props.oninput.call(event);

    rsx! {
        select { oninput, ..props.attributes, {props.children} }
    }
}

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
