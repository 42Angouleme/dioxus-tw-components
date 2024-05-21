use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(id, class)]
pub fn TextArea(
    #[props(extends = textarea)] attributes: Vec<Attribute>,
    #[props(optional)] value: String,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(default)] color: Color,
) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.class);

    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    rsx!(
        textarea {
            ..props.attributes.clone(),
            value: props.value,
            class,
            oninput: oninput,
            id: props.id
        }
    )
}
