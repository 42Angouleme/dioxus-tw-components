use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct RadioProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional, default = false)]
    checked: bool,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,
}

#[component]
pub fn Radio(mut props: RadioProps) -> Element {
    let default_classes = "radio-input";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let oninput = move |event| props.oninput.call(event);

    rsx! {
        input {
            r#type: "radio",
            checked: props.checked,
            oninput,
            ..props.attributes,
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct RadioGroupProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn RadioGroup(mut props: RadioGroupProps) -> Element {
    let default_classes = "radio";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct RadioLabelProps {
    #[props(extends = label, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn RadioLabel(mut props: RadioLabelProps) -> Element {
    let default_classes = "radio-label";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        label { ..props.attributes, {props.children} }
    }
}
