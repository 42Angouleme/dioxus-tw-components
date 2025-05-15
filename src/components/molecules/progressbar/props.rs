use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ProgressBarProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub size: ReadOnlySignal<Size>,

    children: Element,
}

impl std::default::Default for ProgressBarProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            size: ReadOnlySignal::<Size>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ProgressBar(mut props: ProgressBarProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ProgressBarInnerProps {
    #[props(default = 50)]
    progress: u8,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,

    children: Element,
}

impl std::default::Default for ProgressBarInnerProps {
    fn default() -> Self {
        Self {
            progress: 50,
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ProgressBarInner(mut props: ProgressBarInnerProps) -> Element {
    props.update_class_attribute();

    let progress = if props.progress > 100 {
        100
    } else {
        props.progress
    };

    rsx! {
        div { style: "width: {progress}%", ..props.attributes,
            div { {props.children} }
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ProgressLabelProps {
    #[props(default = 50)]
    progress: u8,
    #[props(default = true)]
    show_percentage: bool,

    #[props(extends = span, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for ProgressLabelProps {
    fn default() -> Self {
        Self {
            progress: 50,
            show_percentage: true,
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ProgressLabel(mut props: ProgressLabelProps) -> Element {
    props.update_class_attribute();

    rsx! {
        span {..props.attributes,
            "{props.progress.to_string()}"
            if props.show_percentage {
                "%"
            }
        }
    }
}
