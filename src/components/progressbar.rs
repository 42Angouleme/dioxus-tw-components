use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ProgressBarProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn ProgressBar(mut props: ProgressBarProps) -> Element {
    let default_classes = "progressbar";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ProgressBarInnerProps {
    #[props(default = 50)]
    progress: u8,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn ProgressBarInner(mut props: ProgressBarInnerProps) -> Element {
    let default_classes = "progressbar-inner";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    // Avoid ProgressBar's data-style being applied to ProgressBarInner
    if !props
        .attributes
        .iter_mut()
        .any(|attr| attr.name == "data-style")
    {
        props
            .attributes
            .push(Attribute::new("data-style", "none", None, true));
    }

    let progress = if props.progress > 100 {
        100
    } else {
        props.progress
    };

    rsx! {
        div {
            style: "width: {progress}%",
            ..props.attributes,
            div { {props.children} }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ProgressLabelProps {
    #[props(default = 50)]
    progress: u8,
    #[props(default = true)]
    show_percentage: bool,

    #[props(extends = span, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn ProgressLabel(mut props: ProgressLabelProps) -> Element {
    let default_classes = "progress-label";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        span { ..props.attributes,
            "{props.progress.to_string()}"
            if props.show_percentage {
                "%"
            }
        }
    }
}
