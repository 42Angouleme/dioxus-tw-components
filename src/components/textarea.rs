use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct TextAreaProps {
    #[props(extends = textarea, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    default_value: String,

    #[props(optional)]
    value: Signal<String>,

    #[props(optional)]
    onchange: EventHandler<FormEvent>,
}

#[component]
pub fn TextArea(mut props: TextAreaProps) -> Element {
    let default_classes = "textarea";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let oninput = move |event: FormEvent| {
        props.value.set(event.data.value().clone());
        props.onchange.call(event);
    };

    rsx! {
        textarea {
            oninput,
            value: props.default_value,
            ..props.attributes,
        }
    }
}
