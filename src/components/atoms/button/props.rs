use super::ButtonVariant;
use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ButtonProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub size: ReadOnlySignal<Size>,
    #[props(optional, default)]
    pub variant: ReadOnlySignal<ButtonVariant>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    onmouseenter: EventHandler<MouseEvent>,
    #[props(optional)]
    onmouseleave: EventHandler<MouseEvent>,
    #[props(optional)]
    onfocus: EventHandler<FocusEvent>,

    children: Element,
}

impl std::default::Default for ButtonProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            size: ReadOnlySignal::<Size>::default(),
            variant: ReadOnlySignal::<ButtonVariant>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            onmouseenter: EventHandler::<MouseEvent>::default(),
            onmouseleave: EventHandler::<MouseEvent>::default(),
            onfocus: EventHandler::<FocusEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn Button(mut props: ButtonProps) -> Element {
    props.update_class_attribute();

    let onclick = move |event| props.onclick.call(event);
    let onmouseenter = move |event| props.onmouseenter.call(event);
    let onmouseleave = move |event| props.onmouseleave.call(event);
    let onfocus = move |event| props.onfocus.call(event);

    rsx! {
        button {
            onclick,
            onmouseenter,
            onmouseleave,
            onfocus,
            ..props.attributes,
            {props.children}
        }
    }
}
