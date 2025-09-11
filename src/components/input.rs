use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct InputProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional)]
    value: String,

    #[props(optional)]
    onkeypress: EventHandler<KeyboardEvent>,
    #[props(optional)]
    onblur: EventHandler<FocusEvent>,
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(optional)]
    onmounted: EventHandler<Event<MountedData>>,
}

#[component]
pub fn Input(mut props: InputProps) -> Element {
    let default_classes = "input";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let onkeypress = move |event| props.onkeypress.call(event);
    let onblur = move |event| props.onblur.call(event);
    let oninput = move |event| props.oninput.call(event);

    let onmounted = move |event: Event<MountedData>| props.onmounted.call(event);

    rsx! {
        input {
            onmounted,
            onkeypress,
            onblur,
            oninput,
            value: props.value,
            ..props.attributes,
        }
    }
}
