use dioxus::prelude::*;
use myderive::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(class, id, children)]
pub fn SelectGroup(
    #[props(extends = select)] attributes: Vec<Attribute>,

    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    rsx!(
        select {
            ..props.attributes,
            class: class,
            id: props.id,
            oninput: oninput,
            {props.children}
        }
    )
}

#[props_component(class, id, children)]
pub fn SelectPlaceholder() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        option {
            disabled: true,
            selected: true,
            value: "",
            class: class,
            id: props.id,
            {props.children}
        }
    )
}

#[props_component(class, id, children)]
pub fn SelectLabel(#[props(extends = optgroup)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        optgroup {
            ..props.attributes,
            class: class,
            id: props.id,
            {props.children}
        }
    )
}

#[props_component(class, id, children)]
pub fn SelectItem(#[props(extends = option)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        option {
            ..props.attributes,
            class: class,
            id: props.id,
            {props.children}
        }
    )
}
