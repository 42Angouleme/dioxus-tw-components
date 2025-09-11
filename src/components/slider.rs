use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct SliderProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    
    #[props(optional)]
    value: f64,
    #[props(optional, default = 0.0)]
    min: f64,
    #[props(optional, default = 100.0)]
    max: f64,
    #[props(optional, default = 1.0)]
    step: f64,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,
}

#[component]
pub fn Slider(mut props: SliderProps) -> Element {
    let default_classes = "slider";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let percentage = (props.value - props.min) / (props.max - props.min) * 100.0;

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
                min: props.min,
                max: props.max,
                step: props.step,
                value: props.value,
                oninput: move |event| props.oninput.call(event),
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SliderTicksProps {
    #[props(optional, default = 10.0)]
    step: f64,
    #[props(optional, default = 0.0)]
    min: f64,
    #[props(optional, default = 100.0)]
    max: f64,

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
            for tick in ticks {
                option { value: "{tick}" }
            }
        }
    }
}
