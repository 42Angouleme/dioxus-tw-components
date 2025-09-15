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

    let percentage = (props.value - props.min) / (props.max - props.min) * 100;

    rsx! {
        div { ..props.attributes,
            div { class: "slider-track",
                div {
                    class: "slider-range",
                    style: "width: {percentage}%"
                }
            }
            input {
                class: "slider-thumb",
                r#type: "range",
                min: props.min.to_string(),
                max: props.max.to_string(),
                step: props.step.to_string(),
                value: props.value.to_string(),
                oninput: move |event| props.oninput.call(event),
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SliderTicksProps {
    #[props(optional, default = 10)]
    step: i64,
    #[props(optional, default = 0)]
    min: i64,
    #[props(optional, default = 100)]
    max: i64,

    #[props(extends = datalist, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SliderTicks(mut props: SliderTicksProps) -> Element {
    let default_classes = "slider-ticks";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let mut ticks = Vec::new();
    let mut current = props.min;
    while current <= props.max {
        ticks.push(current);
        current += props.step;
    }

    rsx! {
        datalist { ..props.attributes,
            for i in props.min..props.max {
                if i % props.step == 0 {
                    option { value: i }
                }
            }
            option { value: props.max }
        }
    }
}
