use crate::dioxus_core::IntoAttributeValue;
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

#[derive(Clone, Debug)]
pub struct HoverState {
    is_active: bool,
}

impl HoverState {
    fn new() -> Self {
        Self { is_active: false }
    }

    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    fn open(&mut self) {
        self.is_active = true;
    }

    fn close(&mut self) {
        self.is_active = false;
    }
}

impl IntoAttributeValue for HoverState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn HoverCard(mut props: HoverCardProps) -> Element {
    let mut state = use_context_provider(|| Signal::new(HoverState::new()));

    let default_classes = "hovercard";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div {
            "data-state": state.into_value(),
            onmouseenter: move |_| state.write().open(),
            onmouseleave: move |_| state.write().close(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

#[component]
pub fn HoverCardTrigger(mut props: HoverCardTriggerProps) -> Element {
    let mut state = use_context::<Signal<HoverState>>();

    let default_classes = "hovercard-trigger";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    // We need this event here to not close the hover card when clicking its content
    let onclick = move |event| {
        state.write().toggle();
        props.onclick.call(event);
    };

    rsx! {
        div {
            role: "button",
            "data-state": state.into_value(),
            onclick,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn HoverCardContent(mut props: HoverCardContentProps) -> Element {
    let state = use_context::<Signal<HoverState>>();

    let default_classes = "hovercard-content";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div {
            "data-state": state.into_value(),
            ..props.attributes,
            {props.children}
        }
    }
}
