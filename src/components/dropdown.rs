use crate::dioxus_core::IntoAttributeValue;
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

#[derive(Clone, Copy)]
struct DropdownState {
    is_active: bool,
}

impl DropdownState {
    fn new() -> Self {
        Self { is_active: false }
    }

    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    fn close(&mut self) {
        self.is_active = false;
    }

    fn get_is_active(&self) -> bool {
        self.is_active
    }
}

impl IntoAttributeValue for DropdownState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("open".to_string()),
            false => AttributeValue::Text("closed".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct DropdownProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

/// Usage:
/// ```ignore
/// Dropdown {
///    DropdownToggle {
///        "Dropdown"
///     }
///     DropdownContent {
///       div { "content" }
///    }
/// }
/// ```
/// Use 0 closing_delay_ms to disable the auto close feature
#[component]
pub fn Dropdown(mut props: DropdownProps) -> Element {
    let mut state = use_context_provider(|| Signal::new(DropdownState::new()));

    let default_classes = "dropdown";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { "data-state": state.read().into_value(), ..props.attributes, {props.children} }
        if state.read().get_is_active() {
            div {
                class: "dropdown-backdrop",
                onclick: move |_event| {
                    state.write().close();
                },
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct DropdownToggleProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn DropdownToggle(mut props: DropdownToggleProps) -> Element {
    let mut state = use_context::<Signal<DropdownState>>();

    let default_classes = "button";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        button {
            onclick: move |_| state.write().toggle(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct DropdownContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn DropdownContent(mut props: DropdownContentProps) -> Element {
    let state = use_context::<Signal<DropdownState>>();

    let default_classes = "dropdown-content";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div {
            "data-state": state.read().into_value(),
            ..props.attributes,
            {props.children}
        }
    }
}
