use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(id, class)]
pub fn TextArea(
    #[props(extends = textarea)] attributes: Vec<Attribute>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(default)] color: Color,
) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.class);

    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    // TODO add a default placeholder

    rsx!( textarea { ..props.attributes, class: class, oninput: oninput, id: props.id } )
}
