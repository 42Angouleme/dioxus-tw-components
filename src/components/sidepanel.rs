use crate::components::icon::*;
use crate::dioxus_core::IntoAttributeValue;
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

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

    pub fn open(&mut self) {
        self.is_active = true;
    }

    pub fn close(&mut self) {
        self.is_active = false;
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

#[derive(Clone, PartialEq, Props)]
pub struct SidePanelProps {
    #[props(default = false)]
    is_active: bool,

    children: Element,
}

#[component]
pub fn SidePanel(props: SidePanelProps) -> Element {
    use_context_provider(|| Signal::new(SidePanelState::new(props.is_active)));

    rsx! {
        {props.children}
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SidePanelTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

#[component]
pub fn SidePanelTrigger(mut props: SidePanelTriggerProps) -> Element {
    let mut state = use_context::<Signal<SidePanelState>>();

    let default_classes = "button";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let onclick = move |event: Event<MouseData>| {
        state.write().open();
        props.onclick.call(event)
    };

    rsx! {
        button { onclick, ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SidePanelCloseProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default)]
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
        let default_classes = "sidepanel-close";
        crate::setup_class_attribute(&mut props.attributes, default_classes);
    }

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().close();
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

#[derive(Clone, PartialEq, Props)]
pub struct SidePanelContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SidePanelContent(mut props: SidePanelContentProps) -> Element {
    let state = use_context::<Signal<SidePanelState>>();

    let default_classes = "sidepanel-content";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div {
            "data-state": state.read().into_value(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SidePanelBackgroundProps {
    #[props(optional, default = true)]
    interactive: bool,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

#[component]
pub fn SidePanelBackground(mut props: SidePanelBackgroundProps) -> Element {
    let mut state = use_context::<Signal<SidePanelState>>();

    let default_classes = "sidepanel-background";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        if props.interactive {
            state.write().close();
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
