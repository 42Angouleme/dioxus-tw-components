use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct SliderProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    default_value: i64,
    #[props(optional, default = 0)]
    min: i64,
    #[props(optional, default = 100)]
    max: i64,
    #[props(optional, default = 10)]
    step: i64,

    #[props(optional)]
    value: Signal<i64>,

    #[props(optional)]
    onchange: EventHandler<FormEvent>,
}

#[component]
pub fn Slider(mut props: SliderProps) -> Element {
    let default_classes = "slider";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let oninput = move |event: FormEvent| {
        props.value.set(event.data.value().parse().unwrap_or(0));
        props.onchange.call(event);
    };

    rsx! {
        input {
            class: "slider",
            r#type: "range",
            min: props.min.to_string(),
            max: props.max.to_string(),
            step: props.step.to_string(),
            value: props.default_value.to_string(),
            oninput,
            ..props.attributes
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SliderLabelProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default = 0)]
    value: i64,
    #[props(optional, default = 100)]
    max: i64,
}

#[component]
pub fn SliderLabel(mut props: SliderLabelProps) -> Element {
    let default_classes = "slider-label";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div {..props.attributes,
            {props.value.to_string()}
            " / "
            {props.max.to_string()}
        }
    }
}
