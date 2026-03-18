use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct StepperProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Current step index (0-based)
    pub current_step: usize,
    /// Total number of steps
    pub total_steps: usize,
    /// Title for each step
    pub step_titles: Vec<String>,
    /// Which steps are completed
    #[props(default)]
    pub completed_steps: Vec<bool>,
    /// Called when "Next" is clicked
    pub on_next: EventHandler<()>,
    /// Called when "Back" is clicked
    pub on_back: EventHandler<()>,
    /// Called when "Complete" is clicked (last step)
    #[props(optional)]
    pub on_complete: Option<EventHandler<()>>,
    /// Whether to show "Draft saved" indicator
    #[props(default)]
    pub show_saved: bool,

    children: Element,
}

#[component]
pub fn Stepper(mut props: StepperProps) -> Element {
    let default_classes = "stepper";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let is_first = props.current_step == 0;
    let is_last = props.current_step >= props.total_steps.saturating_sub(1);
    let step_title = props
        .step_titles
        .get(props.current_step)
        .cloned()
        .unwrap_or_default();

    rsx! {
        div {..props.attributes,
            // Progress bar
            div { class: "stepper-progress",
                for i in 0..props.total_steps {
                    {
                        let state = if props.completed_steps.get(i).copied().unwrap_or(false) {
                            "completed"
                        } else if i == props.current_step {
                            "current"
                        } else {
                            "pending"
                        };
                        rsx! {
                            div {
                                class: "stepper-progress-segment",
                                "data-state": state,
                            }
                        }
                    }
                }
            }

            // Header
            div { class: "stepper-header",
                p { class: "stepper-step-label",
                    "Step {props.current_step + 1} of {props.total_steps}"
                }
                h2 { class: "stepper-step-title",
                    "{step_title}"
                }
            }

            // Content
            div { class: "stepper-content",
                {props.children}
            }

            // Navigation
            div { class: "stepper-nav",
                if !is_first {
                    button {
                        class: "button",
                        "data-variant": "outline",
                        onclick: move |_| props.on_back.call(()),
                        "Back"
                    }
                } else {
                    div {}
                }
                div { class: "flex items-center gap-3",
                    if props.show_saved {
                        span { class: "stepper-save-indicator",
                            "Draft saved"
                        }
                    }
                    if is_last {
                        if let Some(on_complete) = &props.on_complete {
                            button {
                                class: "button",
                                "data-variant": "default",
                                onclick: {
                                    let on_complete = on_complete.clone();
                                    move |_| on_complete.call(())
                                },
                                "Complete"
                            }
                        }
                    } else {
                        button {
                            class: "button",
                            "data-variant": "default",
                            onclick: move |_| props.on_next.call(()),
                            "Next"
                        }
                    }
                }
            }
        }
    }
}
