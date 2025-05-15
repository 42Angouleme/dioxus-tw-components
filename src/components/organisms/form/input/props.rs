use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Default, Clone, PartialEq, Props, UiComp)]
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

    #[props(default)]
    pub size: ReadOnlySignal<Size>,
    #[props(default)]
    pub color: ReadOnlySignal<Color>,
}

#[component]
pub fn Input(mut props: InputProps) -> Element {
    props.update_class_attribute();

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
