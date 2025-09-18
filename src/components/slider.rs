use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct SliderProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    value: i64,
    #[props(optional, default = 0)]
    min: i64,
    #[props(optional, default = 100)]
    max: i64,
    #[props(optional, default = 10)]
    step: i64,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,
}

#[component]
pub fn Slider(mut props: SliderProps) -> Element {
    let default_classes = "slider";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        input {
            class: "slider",
            r#type: "range",
            min: props.min.to_string(),
            max: props.max.to_string(),
            step: props.step.to_string(),
            value: props.value.to_string(),
            oninput: move |event| props.oninput.call(event),
            ..props.attributes
        }
    }
}
