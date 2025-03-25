use crate::{attributes::*, components::atoms::icon::*, hooks::use_unique_id};
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ToasterProps {
    #[props(extends = ol, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for ToasterProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

/// The toaster must wrap around your App as high as possible to be used
pub fn Toaster(mut props: ToasterProps) -> Element {
    props.update_class_attribute();

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
    fn color(&mut self, color: Color) -> &mut Self;
    fn title(&mut self, title: impl ToString) -> &mut Self;
    fn duration_in_ms(&mut self, duration: u32) -> &mut Self;
    fn animation(&mut self, animation: Animation) -> &mut Self;
    fn is_closable(&mut self, is_closable: bool) -> &mut Self;
    fn success(&mut self, description: impl ToString);
    fn error(&mut self, description: impl ToString);
    fn loading(&mut self, description: impl ToString);
    fn render(&mut self);
}

impl ToastRenderer for Signal<ToasterState> {
    fn description(&mut self, description: Element) -> &mut Self {
        let shape = self.read().shape.clone();
        self.write().shape = shape.description(description);
        self
    }

    fn color(&mut self, color: Color) -> &mut Self {
        let shape = self.read().shape.clone();
        self.write().shape = shape.color(color);
        self
    }

    fn title(&mut self, title: impl ToString) -> &mut Self {
        let shape = self.read().shape.clone();
        self.write().shape = shape.title(title);
        self
    }

    fn duration_in_ms(&mut self, duration: u32) -> &mut Self {
        let shape = self.read().shape.clone();
        self.write().shape = shape.duration_in_ms(duration);
        self
    }

    fn animation(&mut self, animation: Animation) -> &mut Self {
        let shape = self.read().shape.clone();
        self.write().shape = shape.animation(animation);
        self
    }

    fn is_closable(&mut self, is_closable: bool) -> &mut Self {
        let shape = self.read().shape.clone();
        self.write().shape = shape.is_closable(is_closable);
        self
    }

    /// Build a toast with success background color and title "Success"
    /// The string passed as argument will be the description of the Toast
    fn success(&mut self, description: impl ToString) {
        let mut toast = Toast::default();
        toast = toast
            .title(String::from("Success"))
            .color(Color::Success)
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
        let mut toast = Toast::default();
        toast = toast
            .title(String::from("Error"))
            .color(Color::Destructive)
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
        let mut toast = Toast::default();
        toast = toast
            .title(String::from("Loading"))
            .color(Color::Primary)
            .description(rsx! {
                p { "{description.to_string()}" }
            });
        self.set(ToasterState {
            toast: Some(toast),
            shape: Toast::default(),
        });
    }

    fn render(&mut self) {
        let shape = self.read().shape.clone();
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
#[derive(Clone, Debug, PartialEq, UiComp)]
pub struct Toast {
    id: String,
    title: String,
    description: Element,
    duration_in_ms: u32,
    is_closable: bool,
    pub color: Color,
    pub animation: Animation,
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
            color: Color::default(),
            animation: Animation::default(),
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

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn animation(mut self, animation: Animation) -> Self {
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
fn ToastView(mut state: Signal<ToasterState>, toast: ReadOnlySignal<Toast>) -> Element {
    let class = toast.read().build_class();

    let mut toast_state = use_signal(|| ToastState::Opening);

    let duration_in_ms = toast.read().duration_in_ms;
    let toast_animation = toast.read().animation;

    // This is to animate the Toast in and out
    use_future(move || async move {
        if toast_animation != Animation::None {
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
            #[cfg(target_arch = "wasm32")]
            {
                TimeoutFuture::new(duration_in_ms - animation_play_time).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = tokio::time::sleep(std::time::Duration::from_millis(
                    (duration_in_ms - animation_play_time) as u64,
                ))
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
            class,
            id: "{toast.read().id}",
            "data-state": toast_state.read().to_string(),
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
            class: "absolute top-4 right-4 rounded-global-radius hidden group-hover:block transition-colors focus:outline-hidden focus:ring-3 focus:ring-foreground",
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
            Icon { size: Size::Xs, icon: Icons::Close }
        }
    }
}

/// Hook that returns the ToasterState to spawn a Toast
pub fn use_toast() -> Signal<ToasterState> {
    // Will panic if no Toaster {} upper in the DOM
    use_context::<Signal<ToasterState>>()
}
