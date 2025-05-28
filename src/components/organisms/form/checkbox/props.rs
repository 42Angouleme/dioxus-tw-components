use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct CheckboxProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional, into, default)]
    checked: ReadOnlySignal<bool>,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub size: ReadOnlySignal<Size>,
}

#[component]
pub fn Checkbox(mut props: CheckboxProps) -> Element {
    props.update_class_attribute();

    let oninput = move |event| props.oninput.call(event);

    rsx! {
        input {
            r#type: "checkbox",
            oninput,
            checked: *props.checked.read(),
            ..props.attributes,
        }
    }
}
