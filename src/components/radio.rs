use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct RadioGroupCtx {
    value: Signal<String>,
    onchange: EventHandler<MouseEvent>,
}

#[derive(Clone, PartialEq, Props)]
pub struct RadioGroupProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,

    #[props(optional)]
    default_value: String,

    #[props(optional)]
    value: Signal<String>,

    #[props(optional)]
    onchange: EventHandler<MouseEvent>,
}

#[component]
pub fn RadioGroup(mut props: RadioGroupProps) -> Element {
    use_context_provider(|| RadioGroupCtx {
        value: props.value,
        onchange: props.onchange,
    });

    let default_classes = "radio";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct RadioItemProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    value: String,
}

#[component]
pub fn RadioItem(mut props: RadioItemProps) -> Element {
    let mut state = use_context::<RadioGroupCtx>();

    let default_classes = "radio-input";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let value = props.value.clone();
    let checked = use_memo(move || (state.value)() == value);

    rsx! {
        input {
            r#type: "radio",
            checked,
            onclick: move |event| {
                state.value.set(props.value.clone());
                state.onchange.call(event);
            },
            ..props.attributes,
        }
    }
}
