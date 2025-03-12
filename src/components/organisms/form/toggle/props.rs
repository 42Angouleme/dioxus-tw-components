use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use dioxus_tw_components_macro::UiComp;

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct ToggleProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional)]
    checked: Option<bool>,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub size: ReadOnlySignal<Size>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
}

// Specifically stylised input type checkbox
// The input use the tailwind peer class, you can use at your advantage to style the children
// eg peer-disabled:font-mute will change children text-color when the input is disabled (Label component already does this by default)
pub fn Toggle(mut props: ToggleProps) -> Element {
    props.update_class_attribute();

    let mut interior_sig = use_signal(|| props.checked.unwrap_or_default());

    let onclick = move |event| {
        interior_sig.toggle();
        props.onclick.call(event);
    };

    rsx! {
        button {
            "data-state": match interior_sig() {
                true => AttributeValue::Text("active".to_string()),
                false => AttributeValue::Text("inactive".to_string()),
            },
            r#type: "button",
            onclick,
            ..props.attributes,
        }
    }
}
