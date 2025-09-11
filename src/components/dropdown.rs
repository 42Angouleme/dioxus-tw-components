use chrono::{DateTime, Local, TimeDelta};
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;

#[derive(Clone, Copy)]
struct DropdownState {
    is_active: bool,
    last_hover: DateTime<Local>,
    is_hovered: bool,
    closing_delay_ms: TimeDelta,
}

impl DropdownState {
    fn new(closing_delay_ms: u32) -> Self {
        Self {
            is_active: false,
            last_hover: DateTime::default(),
            is_hovered: false,
            closing_delay_ms: TimeDelta::milliseconds(closing_delay_ms as i64),
        }
    }

    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    fn close(&mut self) {
        self.is_active = false;
    }

    fn get_closing_delay(&self) -> TimeDelta {
        self.closing_delay_ms
    }

    fn get_is_active(&self) -> bool {
        self.is_active
    }

    fn set_last_hover(&mut self, last_hover: DateTime<Local>) {
        self.last_hover = last_hover;
    }

    fn get_last_hover(&self) -> DateTime<Local> {
        self.last_hover
    }

    fn get_is_hovered(&self) -> bool {
        self.is_hovered
    }

    fn set_is_hovered(&mut self, is_hovered: bool) {
        self.is_hovered = is_hovered;
    }
}

impl IntoAttributeValue for DropdownState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct DropdownProps {
    /// Corresponds to the time in ms it takes for the toggle to close itself if not active, 0 disable this feature
    #[props(default = 100)]
    closing_delay_ms: u32,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// Usage:
/// ```ignore
/// Dropdown { closing_delay_ms: 500,
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
    let mut state =
        use_context_provider(|| Signal::new(DropdownState::new(props.closing_delay_ms)));

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
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn DropdownToggle(mut props: DropdownToggleProps) -> Element {
    let mut state = use_context::<Signal<DropdownState>>();

    let default_classes = "dropdown-toggle";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let onclick = move |_: MouseEvent| {
        state.write().toggle();
        state.write().set_last_hover(Local::now());
        state.write().set_is_hovered(true);
    };

    let onmouseleave = move |_| {
        on_mouse_leave(state);
    };

    let onmouseenter = move |_| {
        on_mouse_enter(state);
    };

    rsx! {
        div {
            role: "button",
            "data-state": state.read().into_value(),
            onclick,
            onmouseleave,
            onmouseenter,
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

    let onmouseleave = move |_| {
        on_mouse_leave(state);
    };

    let onmouseenter = move |_| {
        on_mouse_enter(state);
    };

    rsx! {
        div {
            "data-state": state.read().into_value(),
            onmouseleave,
            onmouseenter,
            ..props.attributes,
            {props.children}
        }
    }
}

fn on_mouse_leave(mut state: Signal<DropdownState>) {
    let is_active = state.read().get_is_active();
    let closing_delay = state.read().get_closing_delay();

    spawn(async move {
        if closing_delay <= TimeDelta::zero() || !is_active {
            return;
        }
        state.write().set_is_hovered(false);

        #[cfg(target_arch = "wasm32")]
        {
            TimeoutFuture::new(
                closing_delay
                    .num_milliseconds()
                    .try_into()
                    .unwrap_or_default(),
            )
            .await;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = tokio::time::sleep(std::time::Duration::from_millis(
                closing_delay
                    .num_milliseconds()
                    .try_into()
                    .unwrap_or_default(),
            ))
            .await;
        }

        let is_hovered = state.read().get_is_hovered();

        let last_hover = state.read().get_last_hover();
        let now = Local::now();
        let dt = state.read().get_closing_delay();

        if now - last_hover >= dt && !is_hovered {
            state.write().close();
        }
    });
}

fn on_mouse_enter(mut state: Signal<DropdownState>) {
    state.write().set_last_hover(Local::now());
    state.write().set_is_hovered(true);
}
