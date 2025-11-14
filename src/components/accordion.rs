use crate::components::icon::*;
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

struct AccordionState {
    multi_open: bool,
    active_items: Vec<String>,
}

impl AccordionState {
    fn new(multi_open: bool) -> Self {
        Self {
            multi_open,
            active_items: Vec::with_capacity(1),
        }
    }

    fn add_id(&mut self, id: String) {
        self.active_items.push(id);
    }

    fn remove_id(&mut self, id: String) {
        self.active_items.retain(|x| x != &id);
    }

    fn set_id(&mut self, id: String) {
        self.active_items.clear();
        self.active_items.push(id);
    }

    fn is_active(&self, id: &str) -> bool {
        self.active_items.contains(&id.to_string())
    }

    fn is_active_to_attr_value(&self, id: String) -> AttributeValue {
        match self.active_items.contains(&id) {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Control if multiple items can be open at the same time
    #[props(default = false)]
    multi_open: bool,

    children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    use_context_provider(|| Signal::new(AccordionState::new(props.multi_open)));

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionItemProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// Wrapper for the [AccordionTrigger] and [AccordionContent] components
#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionTriggerProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadSignal<String>,

    /// Determine if the accordion item is open by default
    #[props(optional, into)]
    is_open: ReadSignal<bool>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    /// Decoration element that is displayed next to the trigger text, by default a chevron
    #[props(optional, default = default_trigger_decoration())]
    trigger_decoration: Element,

    children: Element,
}

/// The clickable element that toggles the visibility of the [AccordionContent] component
#[component]
pub fn AccordionTrigger(mut props: AccordionTriggerProps) -> Element {
    let mut state = use_context::<Signal<AccordionState>>();

    use_effect(move || {
        if *props.is_open.read() {
            if !state.peek().multi_open {
                state.write().set_id(props.id.read().clone());
            } else {
                state.write().add_id(props.id.read().clone());
            }
        } else if state.peek().is_active(&props.id.read()) {
            state.write().remove_id(props.id.read().clone());
        }
    });

    let button_closure = move |event: Event<MouseData>| {
        // If the current item is active, remove it from the list, effectively closing it
        if state.read().is_active(&props.id.read()) {
            state.write().remove_id(props.id.read().clone());
        } else {
            // If the current item is not active
            // set it as the only active item if multi_open is false
            // or add it to the list of active items if multi_open is true
            if !state.read().multi_open {
                state.write().set_id(props.id.read().clone());
            } else {
                state.write().add_id(props.id.read().clone());
            }
        }
        props.onclick.call(event)
    };

    let default_classes = "accordion-trigger";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        button {
            "data-state": state.read().is_active_to_attr_value(props.id.read().to_string()),
            onclick: button_closure,
            ..props.attributes,
            {props.children}
            {props.trigger_decoration}
        }
    }
}

fn default_trigger_decoration() -> Element {
    rsx! {
        Icon { class: "default-accordion-trigger", icon: Icons::ExpandMore }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadSignal<String>,

    #[props(optional)]
    height: ReadSignal<String>,

    children: Element,
}

/// Collapsible element that is toggled by the [AccordionTrigger] component
#[component]
pub fn AccordionContent(mut props: AccordionContentProps) -> Element {
    // This is the height of the element when visible, we need to calcul it before rendering it to have a smooth transition
    let mut elem_height = use_signal(|| "".to_string());

    let state = use_context::<Signal<AccordionState>>();

    let final_height = match state.read().is_active(&props.id.read()) {
        true => {
            if props.height.read().is_empty() {
                elem_height()
            } else {
                props.height.read().clone()
            }
        }
        false => "0".to_string(),
    };

    let attributes = props.attributes.clone();

    let default_classes = "accordion-content";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div {
            onmounted: move |element| {
                let attributes = attributes.clone();
                async move {
                    if !props.height.read().is_empty() {
                        return;
                    }
                    if attributes
                        .iter()
                        .any(|attr| attr.name == "data-animation")
                    {
                        elem_height
                            .set(
                                match element.data().get_scroll_size().await {
                                    Ok(size) => format!("{}px", size.height),
                                    Err(e) => {
                                        dioxus::logger::tracing::error!(
                                            "AccordionContent: Failed to get element height(id probably not set): setting it to auto: {e:?}",
                                        );
                                        "auto".to_string()
                                    }
                                },
                            );
                    } else {
                        elem_height.set("auto".to_string());
                    }
                }
            },
            "data-state": state.read().is_active_to_attr_value(props.id.read().to_string()),
            id: props.id,
            height: final_height,
            ..props.attributes,
            {props.children}
        }
    }
}
