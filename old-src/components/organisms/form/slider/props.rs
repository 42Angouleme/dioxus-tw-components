use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct SliderProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional)]
    value: String,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(optional)]
    onmounted: EventHandler<Event<MountedData>>,

    #[props(default)]
    pub color: ReadOnlySignal<Color>,
}

#[component]
pub fn Slider(mut props: SliderProps) -> Element {
    props.update_class_attribute();

    let oninput = move |event| props.oninput.call(event);

    let onmounted = move |event: Event<MountedData>| props.onmounted.call(event);

    rsx! {
        input {
            onmounted,
            oninput,
            r#type: "range",
            value: props.value,
            ..props.attributes,
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
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

impl std::default::Default for SliderTicksProps {
    fn default() -> Self {
        Self {
            step: 10,
            min: 0,
            max: 100,
            attributes: Vec::<Attribute>::default(),
        }
    }
}

#[component]
pub fn SliderTicks(mut props: SliderTicksProps) -> Element {
    props.update_class_attribute();

    rsx! {
        datalist {..props.attributes,
            for i in props.min..props.max {
                if i % props.step == 0 {
                    option { value: i }
                }
            }
            option { value: props.max }
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SliderLabelProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default = 0)]
    value: i64,
    #[props(optional, default = 100)]
    max: i64,
}

impl std::default::Default for SliderLabelProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            value: 0,
            max: 100,
        }
    }
}

#[component]
pub fn SliderLabel(mut props: SliderLabelProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div {..props.attributes,
            {props.value.to_string()}
            " / "
            {props.max.to_string()}
        }
    }
}
