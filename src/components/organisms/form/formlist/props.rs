use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

pub struct FormListState {
    max_size: usize,
    current_size: usize,
}

impl FormListState {
    fn new(current_size: usize) -> Self {
        FormListState {
            max_size: 1,
            current_size,
        }
    }

    fn get_max_size(&self) -> usize {
        self.max_size
    }

    fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
    }

    fn get_current_size(&self) -> usize {
        self.current_size
    }

    fn add_one(&mut self) {
        if self.current_size < self.max_size {
            self.current_size += 1;
        }
    }

    fn remove_one(&mut self) {
        if self.current_size > 1 {
            self.current_size -= 1;
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct FormListProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = 1)]
    max_size: usize,
    // Size of non-empty fields in the list
    #[props(default = 1)]
    current_size: usize,

    children: Element,
}

impl std::default::Default for FormListProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            max_size: 1,
            current_size: 1,
            children: rsx! {},
        }
    }
}

pub fn FormList(mut props: FormListProps) -> Element {
    use_context_provider(|| Signal::new(FormListState::new(props.current_size)));

    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct FormListTriggerPlusProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn FormListTriggerPlus(mut props: FormListTriggerPlusProps) -> Element {
    let mut state = use_context::<Signal<FormListState>>();

    props.update_class_attribute();

    rsx! {
        div {
            onclick: move |_| {
                state.write().add_one();
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct FormListTriggerMinusProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for FormListTriggerMinusProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

pub fn FormListTriggerMinus(mut props: FormListTriggerMinusProps) -> Element {
    let mut state = use_context::<Signal<FormListState>>();

    props.update_class_attribute();

    rsx! {
        div {
            onclick: move |_| {
                state.write().remove_one();
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct FormListContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default)]
    list_fields: Vec<Element>,
}

pub fn FormListContent(mut props: FormListContentProps) -> Element {
    let mut state = use_context::<Signal<FormListState>>();

    props.update_class_attribute();

    let max_size = props.list_fields.len();
    use_effect(move || {
        state.write().set_max_size(max_size);
    });

    let fields = props
        .list_fields
        .iter()
        .take(state.read().get_current_size())
        .map(|field| {
            rsx! {
                {field.clone()}
            }
        });

    rsx! {
        div { ..props.attributes,{fields} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct FormListMaxSizeProps {}

pub fn FormListMaxSize() -> Element {
    let state = use_context::<Signal<FormListState>>();

    rsx! { "{state.read().get_max_size()}" }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct FormListCurrentSizeProps {}

pub fn FormListCurrentSize() -> Element {
    let state = use_context::<Signal<FormListState>>();

    rsx! { "{state.read().get_current_size()}" }
}
