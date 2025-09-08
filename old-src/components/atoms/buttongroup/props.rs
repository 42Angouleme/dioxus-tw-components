use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ButtonGroupProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub size: ReadOnlySignal<Size>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,

    children: Element,
}

impl std::default::Default for ButtonGroupProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            size: ReadOnlySignal::<Size>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            children: rsx! {},
        }
    }
}

#[derive(Default, Clone)]
struct FieldData {
    pub color: ReadOnlySignal<Color>,
    pub size: ReadOnlySignal<Size>,
    pub animation: ReadOnlySignal<Animation>,
}

#[component]
pub fn ButtonGroup(mut props: ButtonGroupProps) -> Element {
    props.update_class_attribute();
    let _class = use_context_provider(|| {
        let data = FieldData {
            color: props.color,
            size: props.size,
            animation: props.animation,
        };
        Signal::new(data)
    });

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ButtonGroupItemProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub(crate) color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub(crate) size: ReadOnlySignal<Size>,
    #[props(optional, default)]
    pub(crate) animation: ReadOnlySignal<Animation>,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    onmouseenter: EventHandler<MouseEvent>,
    #[props(optional)]
    onmouseleave: EventHandler<MouseEvent>,
    #[props(optional)]
    onfocus: EventHandler<FocusEvent>,

    children: Element,
}

impl std::default::Default for ButtonGroupItemProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            size: ReadOnlySignal::<Size>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            onmouseenter: EventHandler::<MouseEvent>::default(),
            onmouseleave: EventHandler::<MouseEvent>::default(),
            onfocus: EventHandler::<FocusEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ButtonGroupItem(mut props: ButtonGroupItemProps) -> Element {
    let data = use_context::<Signal<FieldData>>();
    props.color = data.read().color;
    props.size = data.read().size;
    props.animation = data.read().animation;

    props.update_class_attribute();

    let onclick = move |event| props.onclick.call(event);
    let onmouseenter = move |event| props.onmouseenter.call(event);
    let onmouseleave = move |event| props.onmouseleave.call(event);
    let onfocus = move |event| props.onfocus.call(event);

    rsx! {
        button {
            onclick,
            onmouseenter,
            onmouseleave,
            onfocus,
            ..props.attributes,
            {props.children}
        }
    }
}
