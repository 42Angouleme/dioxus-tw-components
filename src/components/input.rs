use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct InputProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default)]
    default_value: String,

    #[props(default)]
    value: Signal<String>,

    #[props(default)]
    onchange: EventHandler<FormEvent>,
}

#[component]
pub fn Input(mut props: InputProps) -> Element {
    let default_classes = "input";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let oninput = move |event: FormEvent| {
        props.value.set(event.data.value().clone());
        props.onchange.call(event);
    };

    rsx! {
        input {
            oninput,
            value: props.default_value,
            ..props.attributes,
        }
    }
}
