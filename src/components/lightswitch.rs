use crate::components::icon::*;
use dioxus::prelude::*;
use serde_json::Value;

pub struct LightSwitchState {
    active: bool,
}

impl LightSwitchState {
    pub fn new(active: bool) -> Self {
        Self { active }
    }

    fn toggle(&mut self) -> bool {
        self.active = !self.active;
        self.active
    }

    fn get_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct LightSwitchProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    children: Element,
}

/// This component inserts/remove "dark" in the DOM on the div with id of main
#[component]
pub fn LightSwitch(mut props: LightSwitchProps) -> Element {
    let default_classes = "lightswitch";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let storage_dark_theme = use_resource(move || async move {
        // Get dark_theme from localStorage, if not found add it to false
        let mut eval = document::eval(
            r#"
            var dark_theme = localStorage.getItem("dark_theme");
            if (dark_theme == null) {
                var dark_theme = false;
                localStorage.setItem("dark_theme", dark_theme);
            } else {
                let main_div = document.getElementById("main");
                if (main_div != null && dark_theme == "true") {
                    main_div.classList.add("dark");
                }
            }
            dioxus.send(dark_theme);
            "#,
        );

        eval.recv().await
    });

    let mut state = use_signal(|| LightSwitchState::new(false));

    use_effect(move || {
        if let Some(Ok(Value::String(str))) = &*storage_dark_theme.read_unchecked() {
            let parsed_str = str.parse::<bool>();
            if let Ok(bool_value) = parsed_str {
                state.write().set_active(bool_value);
            }
        };
    });

    let mut onclick = move |_| {
        let dark_theme = state.write().toggle();
        spawn(async move {
            // Change value of dark_theme in localStorage
            let eval = document::eval(
                r#"
                const dark_theme = await dioxus.recv();
                localStorage.setItem("dark_theme", dark_theme);
                let main_div = document.getElementById("main");
                if (main != null) {
                    if (dark_theme) {
                        main_div.classList.add("dark");
                    } else {
                        main_div.classList.remove("dark");
                    }
                }
                "#,
            );
            let _ = eval.send(dark_theme);
        });
    };

    let icon = if state.read().get_active() {
        rsx! {
            Icon { icon: Icons::DarkMode }
        }
    } else {
        rsx! {
            Icon { icon: Icons::LightMode }
        }
    };

    rsx! {
        button {
            r#type: "button",
            onclick: move |e| {
                match props.onclick {
                    Some(p) => {
                        state.write().toggle();
                        p.call(e);
                    }
                    None => onclick(e),
                }
            },
            ..props.attributes,
            {icon}
        }
    }
}
