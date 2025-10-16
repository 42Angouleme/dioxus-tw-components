use dioxus::prelude::*;

struct TabsState(String);

#[derive(Clone, PartialEq, Props)]
pub struct TabsProps {
    #[props(optional)]
    default_tab: ReadSignal<String>,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    use_context_provider(|| Signal::new(TabsState(props.default_tab.read().clone())));

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsListProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TabsList(mut props: TabsListProps) -> Element {
    let default_classes = "tabs-list";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsTriggerProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadSignal<String>,

    children: Element,
}

#[component]
pub fn TabsTrigger(mut props: TabsTriggerProps) -> Element {
    let mut state = use_context::<Signal<TabsState>>();

    let default_classes = "tabs-trigger";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let is_active = state.read().0 == *props.id.read();

    let onclick = move |_: MouseEvent| {
        state.write().0 = props.id.read().clone();
    };

    rsx! {
        button {
            "data-state": if is_active { "active" } else { "inactive" },
            onclick,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadSignal<String>,

    children: Element,
}

#[component]
pub fn TabsContent(mut props: TabsContentProps) -> Element {
    let state = use_context::<Signal<TabsState>>();

    let default_classes = "tabs-content";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let is_active = state.read().0 == *props.id.read();

    rsx! {
        div {
            "data-state": if is_active { "active" } else { "inactive" },
            ..props.attributes,
            {props.children}
        }
    }
}
