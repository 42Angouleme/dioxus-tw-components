use dioxus::prelude::*;
use myderive::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(class, id, children)]
pub fn Dropdown(#[props(default)] open: bool) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn DropdownToggle() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

// Use HTML/CSS black magic to render the dropdown content
// group-focus-within:block
#[props_component(class, id, children)]
pub fn DropdownContent() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}
