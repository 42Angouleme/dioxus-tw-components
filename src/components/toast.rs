use crate::components::icon::*;
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;

#[derive(Clone, PartialEq)]
pub struct ToastItem {
    id: String,
    title: String,
    description: String,
    duration: u32,
    is_visible: bool,
}

impl ToastItem {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: format!("toast-{}", rand::random::<u32>()),
            title,
            description,
            duration: 5000,
            is_visible: true,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct ToasterState {
    toasts: Vec<ToastItem>,
}

impl ToasterState {
    pub fn add_toast(&mut self, toast: ToastItem) {
        self.toasts.push(toast);
    }

    pub fn remove_toast(&mut self, id: String) {
        self.toasts.retain(|t| t.id != id);
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ToasterProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// The toaster must wrap around your App as high as possible to be used
#[component]
pub fn Toaster(mut props: ToasterProps) -> Element {
    let default_classes = "toast";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let state = use_context_provider::<Signal<ToasterState>>(|| Signal::new(ToasterState::default()));

    rsx! {
        {props.children}
        div { ..props.attributes,
            for toast in &state.read().toasts {
                ToastView { 
                    key: "{toast.id}",
                    toast: toast.clone() 
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ToastViewProps {
    toast: ToastItem,
}

#[component]
pub fn ToastView(props: ToastViewProps) -> Element {
    let mut state = use_context::<Signal<ToasterState>>();
    let toast_id = props.toast.id.clone();

    // Auto-remove toast after duration
    use_effect(move || {
        let id = toast_id.clone();
        spawn(async move {
            #[cfg(target_arch = "wasm32")]
            {
                TimeoutFuture::new(props.toast.duration).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                tokio::time::sleep(std::time::Duration::from_millis(props.toast.duration as u64)).await;
            }
            state.write().remove_toast(id);
        });
    });

    let close_toast = {
        let id = props.toast.id.clone();
        move |_: MouseEvent| {
            state.write().remove_toast(id.clone());
        }
    };

    rsx! {
        div {
            class: "toast-item",
            "data-state": if props.toast.is_visible { "active" } else { "inactive" },
            div { class: "toast-title", "{props.toast.title}" }
            div { class: "toast-description", "{props.toast.description}" }
            button {
                class: "toast-close",
                onclick: close_toast,
                Icon { icon: Icons::Close }
            }
        }
    }
}

pub trait ToastRenderer {
    fn success(&mut self, title: impl ToString, description: impl ToString);
    fn error(&mut self, title: impl ToString, description: impl ToString);
    fn info(&mut self, title: impl ToString, description: impl ToString);
}

impl ToastRenderer for Signal<ToasterState> {
    fn success(&mut self, title: impl ToString, description: impl ToString) {
        let toast = ToastItem::new(title.to_string(), description.to_string());
        self.write().add_toast(toast);
    }

    fn error(&mut self, title: impl ToString, description: impl ToString) {
        let toast = ToastItem::new(title.to_string(), description.to_string());
        self.write().add_toast(toast);
    }

    fn info(&mut self, title: impl ToString, description: impl ToString) {
        let toast = ToastItem::new(title.to_string(), description.to_string());
        self.write().add_toast(toast);
    }
}
