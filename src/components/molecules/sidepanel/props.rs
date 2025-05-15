use crate::{attributes::*, components::atoms::icon::*};
use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, Copy)]
pub struct SidePanelState {
    is_active: bool,
}

impl SidePanelState {
    fn new(is_active: bool) -> Self {
        Self { is_active }
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }
}

impl IntoAttributeValue for SidePanelState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelProps {
    #[props(default = false)]
    is_active: bool,

    children: Element,
}

impl std::default::Default for SidePanelProps {
    fn default() -> Self {
        Self {
            is_active: false,
            children: rsx! {},
        }
    }
}

#[component]
pub fn SidePanel(props: SidePanelProps) -> Element {
    use_context_provider(|| Signal::new(SidePanelState::new(props.is_active)));

    rsx! {
        {props.children}
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for SidePanelTriggerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn SidePanelTrigger(mut props: SidePanelTriggerProps) -> Element {
    let mut state = use_context::<Signal<SidePanelState>>();

    props.update_class_attribute();

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().toggle();
        props.onclick.call(event)
    };

    rsx! {
        div { onclick, ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelCloseProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    children: Element,
}

impl std::default::Default for SidePanelCloseProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: Ok(VNode::default()), // Default this way to be able to check the children in SidePanelClose
        }
    }
}

/// Div to close the content side panel, by default it is a cross located at the top left corner of the side panel
/// If you provide a children, it will be used instead of the default cross and no internal styling will be provided
#[component]
pub fn SidePanelClose(mut props: SidePanelCloseProps) -> Element {
    let mut state = use_context::<Signal<SidePanelState>>();

    let has_children = props.children != Ok(VNode::default());

    if !has_children {
        props.update_class_attribute();
    }

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().is_active = false;
    };

    rsx! {
        div { onclick, ..props.attributes,
            if !has_children {
                Icon { icon: Icons::Close }
            } else {
                {props.children}
            }
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
    #[props(optional, default)]
    pub side: ReadOnlySignal<Side>,

    children: Element,
}

impl std::default::Default for SidePanelContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            side: ReadOnlySignal::<Side>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn SidePanelContent(mut props: SidePanelContentProps) -> Element {
    let state = use_context::<Signal<SidePanelState>>();

    props.update_class_attribute();

    rsx! {
        div { "data-state": state.read().into_value(), ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelBackgroundProps {
    #[props(optional, default = true)]
    interactive: bool,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for SidePanelBackgroundProps {
    fn default() -> Self {
        Self {
            interactive: true,
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn SidePanelBackground(mut props: SidePanelBackgroundProps) -> Element {
    let mut state = use_context::<Signal<SidePanelState>>();

    props.update_class_attribute();

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        if props.interactive {
            state.write().is_active = false;
            props.onclick.call(event)
        }
    };

    rsx! {
        div {
            "data-state": state.read().into_value(),
            onclick,
            ..props.attributes,
            {props.children}
        }
    }
}
