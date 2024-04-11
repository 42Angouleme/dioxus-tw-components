use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

struct TabsState(String);

props!(TabsProps {
    #[props(into)]
    default_tab: String,
});

impl Component for TabsProps {
    fn view(self) -> Element {
        use_context_provider(|| Signal::new(TabsState(self.default_tab)));

        let class = super::style::TabsClass::builder().with_class(self.class);

        rsx!(
            div { class: class, { self.children } }
        )
    }
}

props!(TabsListProps {});

impl Component for TabsListProps {
    fn view(self) -> Element {
        let class = super::style::TabsListClass::builder().with_class(self.class);

        rsx!(
            div { class: class, { self.children } }
        )
    }
}

props!(TabsTriggerProps {});

impl Component for TabsTriggerProps {
    fn view(self) -> Element {
        let mut tab_state = consume_context::<Signal<TabsState>>();

        let class = super::style::TabsTriggerClass::builder().with_class(self.class);

        let state = match tab_state.read().0 == self.id {
            true => "active",
            false => "inactive",
        };

        let onclick = move |_| {
            tab_state.set(TabsState(self.id.clone()));
        };

        rsx!(
            // data-state is used to style the button
            button { "data-state": state, class: class, onclick: onclick, { self.children } }
        )
    }
}

props!(TabsContentProps {});

impl Component for TabsContentProps {
    fn view(self) -> Element {
        let tab_state = consume_context::<Signal<TabsState>>();

        let class = super::style::TabsContentClass::builder().with_class(self.class);

        rsx!(
            div { class: class, hidden: if tab_state.read().0 == self.id { false } else { true }, { self.children } }
        )
    }
}
