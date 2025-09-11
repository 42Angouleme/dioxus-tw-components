use dioxus::prelude::*;

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

#[derive(Clone, PartialEq, Props)]
pub struct FormListProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = 1)]
    max_size: usize,
    #[props(default = 1)]
    current_size: usize,

    children: Element,
}

#[component]
pub fn FormList(mut props: FormListProps) -> Element {
    let default_classes = "formlist";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let mut state = use_context_provider(|| Signal::new(FormListState::new(props.current_size)));
    
    use_effect(move || {
        state.write().set_max_size(props.max_size);
    });

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct FormListItemProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn FormListItem(mut props: FormListItemProps) -> Element {
    let default_classes = "formlist-item";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct FormListLabelProps {
    #[props(extends = label, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn FormListLabel(mut props: FormListLabelProps) -> Element {
    let default_classes = "formlist-label";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        label { ..props.attributes, {props.children} }
    }
}
