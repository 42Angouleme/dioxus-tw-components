use crate::{components::icon::*, use_unique_id};
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ToastColor {
    #[default]
    Default,
    Primary,
    Secondary,
    Destructive,
    Success,
}

impl std::fmt::Display for ToastColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ToastColor::Default => "default",
                ToastColor::Primary => "primary",
                ToastColor::Secondary => "secondary",
                ToastColor::Destructive => "destructive",
                ToastColor::Success => "success",
            }
        )
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ToastAnimation {
    None,
    Light,
    #[default]
    Full,
}

impl std::fmt::Display for ToastAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ToastAnimation::None => "none",
                ToastAnimation::Light => "light",
                ToastAnimation::Full => "full",
            }
        )
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ToasterProps {
    #[props(extends = ol, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// The toaster must wrap around your App as high as possible to be used
#[component]
pub fn Toaster(mut props: ToasterProps) -> Element {
    let default_classes = "toaster";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let state =
        use_context_provider::<Signal<ToasterState>>(|| Signal::new(ToasterState::default()));

    rsx! {
        {props.children}
        ol { role: "alert", id: "dx-toast", ..props.attributes,
            if let Some(toast) = &state.read().toast {
                ToastView { state, toast: toast.clone() }
            }
        }
    }
}

pub trait ToastRenderer {
    fn description(&mut self, description: Element) -> &mut Self;
    fn color(&mut self, color: ToastColor) -> &mut Self;
    fn title(&mut self, title: impl ToString) -> &mut Self;
    fn duration_in_ms(&mut self, duration: u32) -> &mut Self;
    fn animation(&mut self, animation: ToastAnimation) -> &mut Self;
    fn is_closable(&mut self, is_closable: bool) -> &mut Self;
    fn success(&mut self, description: impl ToString);
    fn error(&mut self, description: impl ToString);
    fn loading(&mut self, description: impl ToString);
    fn render(&mut self);
}

impl ToastRenderer for Signal<ToasterState> {
    fn description(&mut self, description: Element) -> &mut Self {
        let shape = self.peek().shape.clone();
        self.write().shape = shape.description(description);
        self
    }

    fn color(&mut self, color: ToastColor) -> &mut Self {
        let shape = self.peek().shape.clone();
        self.write().shape = shape.color(color);
        self
    }

    fn title(&mut self, title: impl ToString) -> &mut Self {
        let shape = self.peek().shape.clone();
        self.write().shape = shape.title(title);
        self
    }

    fn duration_in_ms(&mut self, duration: u32) -> &mut Self {
        let shape = self.peek().shape.clone();
        self.write().shape = shape.duration_in_ms(duration);
        self
    }

    fn animation(&mut self, animation: ToastAnimation) -> &mut Self {
        let shape = self.peek().shape.clone();
        self.write().shape = shape.animation(animation);
        self
    }

    fn is_closable(&mut self, is_closable: bool) -> &mut Self {
        let shape = self.peek().shape.clone();
        self.write().shape = shape.is_closable(is_closable);
        self
    }

    /// Build a toast with success background color and title "Success"
    /// The string passed as argument will be the description of the Toast
    fn success(&mut self, description: impl ToString) {
        let toast = Toast::default()
            .title(String::from("Success"))
            .color(ToastColor::Success)
            .description(rsx! {
                p { "{description.to_string()}" }
            });
        self.set(ToasterState {
            toast: Some(toast),
            shape: Toast::default(),
        });
    }

    /// Build a toast with destructive background color and title "Error"
    /// The string passed as argument will be the description of the Toast
    fn error(&mut self, description: impl ToString) {
        let toast = Toast::default()
            .title(String::from("Error"))
            .color(ToastColor::Destructive)
            .description(rsx! {
                p { "{description.to_string()}" }
            });
        self.set(ToasterState {
            toast: Some(toast),
            shape: Toast::default(),
        });
    }

    /// Build a toast with primary background color and title "Loading"
    /// The string passed as argument will be the description of the Toast
    fn loading(&mut self, description: impl ToString) {
        let toast = Toast::default()
            .title(String::from("Loading"))
            .color(ToastColor::Primary)
            .description(rsx! {
                p { "{description.to_string()}" }
            });
        self.set(ToasterState {
            toast: Some(toast),
            shape: Toast::default(),
        });
    }

    fn render(&mut self) {
        let shape = self.peek().shape.clone();
        self.set(ToasterState {
            toast: Some(shape),
            shape: Toast::default(),
        });
    }
}

/// Used to keep track of all the current toasts, for now it only keeps 1 Toast
#[derive(Default)]
pub struct ToasterState {
    pub toast: Option<Toast>,
    pub shape: Toast,
}

/// A Toast with a default duration of 10s
#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    id: String,
    title: String,
    description: Element,
    duration_in_ms: u32,
    is_closable: bool,
    pub color: ToastColor,
    pub animation: ToastAnimation,
    state: ToastState,
}

impl std::default::Default for Toast {
    fn default() -> Self {
        Self {
            id: use_unique_id(),
            title: String::default(),
            description: Ok(VNode::default()), // Default this way to be able to check the children
            duration_in_ms: 6_000,
            is_closable: true,
            color: ToastColor::default(),
            animation: ToastAnimation::default(),
            state: ToastState::Opening,
        }
    }
}

impl Toast {
    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn description(mut self, description: Element) -> Self {
        self.description = description;
        self
    }

    pub fn color(mut self, color: ToastColor) -> Self {
        self.color = color;
        self
    }

    pub fn animation(mut self, animation: ToastAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn duration_in_ms(mut self, duration: u32) -> Self {
        self.duration_in_ms = duration;
        self
    }

    pub fn is_closable(mut self, is_closable: bool) -> Self {
        self.is_closable = is_closable;
        self
    }
}

/// Define the state of an individual toast, used to animate the Toast
#[derive(Clone, Debug, PartialEq, Default)]
enum ToastState {
    #[default]
    Opening,
    Open,
    Closing,
    // Close is not needed since it means the Toast does not exist anymore
}

impl std::fmt::Display for ToastState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ToastState::Opening => "opening",
                ToastState::Open => "open",
                ToastState::Closing => "closing",
            }
        )
    }
}

/// Used to render the Toast, also update the ToasterState
#[component]
fn ToastView(mut state: Signal<ToasterState>, toast: ReadSignal<Toast>) -> Element {
    let mut toast_state = use_signal(|| ToastState::Opening);

    let duration_in_ms = toast.read().duration_in_ms;
    let toast_animation = toast.read().animation;

    // This is to animate the Toast in and out
    //   use_effect(move || {
    //       let mut timer = document::eval(&format!(
    //           "setInterval(() => {{
    //               dioxus.send(true);
    //           }}, {});",
    //       ), );
    //   });
    use_future(move || async move {
        if toast_animation != ToastAnimation::None {
            #[cfg(target_arch = "wasm32")]
            {
                TimeoutFuture::new(10).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
            toast_state.set(ToastState::Open);

            let animation_play_time = 150;
            let animation_duration = duration_in_ms.saturating_sub(animation_play_time);
            #[cfg(target_arch = "wasm32")]
            {
                TimeoutFuture::new(animation_duration).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ =
                    tokio::time::sleep(std::time::Duration::from_millis(animation_duration as u64))
                        .await;
            }

            toast_state.set(ToastState::Closing);
            #[cfg(target_arch = "wasm32")]
            {
                TimeoutFuture::new(animation_play_time).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = tokio::time::sleep(std::time::Duration::from_millis(
                    animation_play_time as u64,
                ))
                .await;
            }
        } else {
            #[cfg(target_arch = "wasm32")]
            {
                TimeoutFuture::new(duration_in_ms).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = tokio::time::sleep(std::time::Duration::from_millis(duration_in_ms as u64))
                    .await;
            }
        }

        state.set(ToasterState::default());
    });

    rsx! {
        li {
            class: "toast",
            id: "{toast.read().id}",
            "data-state": toast_state.read().to_string(),
            "data-style": toast.read().color.to_string(),
            "data-animation": toast_animation.to_string(),
            h6 { class: "h6", "{toast.read().title}" }
            if toast.read().is_closable {
                ToastClose { state, toast_state }
            }
            {toast.read().description.clone()}
        }
    }
}

/// Used to add a cross mark to manually close the Toast
/// The Timeout is there to let the animation some time to play
#[component]
fn ToastClose(mut state: Signal<ToasterState>, mut toast_state: Signal<ToastState>) -> Element {
    rsx! {
        button {
            class: "toast-close",
            r#type: "button",
            onclick: move |_| {
                spawn(async move {
                    toast_state.set(ToastState::Closing);
                    #[cfg(target_arch = "wasm32")]
                    {
                        TimeoutFuture::new(150).await;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let _ = tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                    }
                    state.set(ToasterState::default());
                });
            },
            Icon { style: "font-size: 0.75rem;", icon: Icons::Close }
        }
    }
}

/// Hook that returns the ToasterState to spawn a Toast
pub fn use_toast() -> Signal<ToasterState> {
    // Will panic if no Toaster {} upper in the DOM
    use_context::<Signal<ToasterState>>()
}
