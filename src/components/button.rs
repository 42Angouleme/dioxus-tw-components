use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    /// Additional attributes to apply to the element
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The click event handler
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    /// The double click event handler
    #[props(optional)]
    ondoubleclick: EventHandler<MouseEvent>,
    /// The mouse down event handler
    #[props(optional)]
    onmousedown: EventHandler<MouseEvent>,
    /// The mouse up event handler
    #[props(optional)]
    onmouseup: EventHandler<MouseEvent>,

    /// Remove default CSS classes
    #[props(default = false)]
    noclasses: bool,

    /// The children element
    children: Element,
}

#[component]
pub fn Button(mut props: ButtonProps) -> Element {
    let default_classes = "button";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let onclick = move |event| props.onclick.call(event);
    let ondoubleclick = move |event| props.ondoubleclick.call(event);
    let onmousedown = move |event| props.onmousedown.call(event);
    let onmouseup = move |event| props.onmouseup.call(event);

    rsx! {
        button {
            onclick,
            ondoubleclick,
            onmousedown,
            onmouseup,
            ..props.attributes,
            {props.children}
        }
    }
}
