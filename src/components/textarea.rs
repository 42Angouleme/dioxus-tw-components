use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct TextAreaProps {
    #[props(extends = textarea, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional)]
    value: String,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(optional)]
    onmounted: EventHandler<Event<MountedData>>,
}

#[component]
pub fn TextArea(mut props: TextAreaProps) -> Element {
    let default_classes = "textarea";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let oninput = move |event| props.oninput.call(event);

    let onmounted = move |event: Event<MountedData>| props.onmounted.call(event);

    rsx! {
        textarea {
            onmounted,
            oninput,
            value: props.value,
            ..props.attributes,
        }
    }
}
