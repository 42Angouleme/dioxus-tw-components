use dioxus::prelude::*;
use myderive::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(id, class, children)]
pub fn Checkbox(
    #[props(extends = input)] attributes: Vec<Attribute>,
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
        label { class: "cursor-pointer gap-x-1 flex items-center",
            input {
                ..props.attributes,
                r#type: "checkbox",
                class: class,
                oninput: oninput,
                id: props.id
            }
            div { class: "peer-disabled:opacity-30", {props.children} }
        }
    )
}
