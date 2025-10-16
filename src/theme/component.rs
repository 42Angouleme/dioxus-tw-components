use crate::{prelude::*, theme::*};
use dioxus::prelude::*;

#[component]
pub fn ThemePicker() -> Element {
    rsx! {
        SidePanel {
            MiniPicker {}
            SidePanelBackground { style: "z-index: 998;" }
            SidePanelContent {
                class: "theme-picker",
                "data-side": "right",
                ColorPicker {}
            }
        }
    }
}

#[component]
fn ColorPicker() -> Element {
    let selected_color = use_signal(|| String::from("primary"));

    let mut theme_manager = use_context::<Signal<ThemeManager>>();

    let current_theme = theme_manager.read().current_theme;
    let colors = theme_manager.read().themes[current_theme].colors.clone();

    let onchange = move |event: FormEvent| {
        // TODO Very ugly but works

        // Convert the event value to an HslColor struct
        let Ok(hsl_color) = HslColor::try_new_from_hex(&event.data().value()) else {
            return;
        };

        // We check if the selected color is the default one, or if it is a foreground color
        let select_color = if *selected_color.read() == "foreground" {
            "background".to_string()
        } else if selected_color.read().contains("foreground") {
            selected_color.read().replace("-foreground", "")
        } else {
            selected_color.read().to_string()
        };

        // Get the current selected color in the theme manager (as mut ref)
        if let Some(color_choice) = theme_manager.write().themes[current_theme]
            .colors
            .get_mut(&select_color)
        {
            match color_choice {
                ColorChoice::Simple(color) => {
                    *color = hsl_color;
                }
                ColorChoice::Duo(color_bg, color_fg) => {
                    if select_color == "background" && *selected_color.read() == "foreground" {
                        *color_fg = hsl_color;
                        return;
                    }

                    if selected_color.read().contains("foreground") {
                        *color_fg = hsl_color;
                    } else {
                        *color_bg = hsl_color;
                    }
                }
            }
        }
    };

    rsx! {
        SidePanelClose {}
        div { class: "theme-picker-content",
            Input {
                role: "button",
                id: "color-picker-input",
                r#type: "color",
                style: "visibility: hidden;",
                onchange,
            }
            for (str , color) in colors.into_iter() {
                ColorSelector {
                    color_str: str,
                    color: color.clone(),
                    selected_color,
                }
            }
            RadiusSelector {}
            ButtonExport {}
        }
    }
}

#[component]
fn ColorSelector(
    color_str: ReadSignal<String>,
    color: ColorChoice,
    mut selected_color: Signal<String>,
) -> Element {
    let content: Element = match color {
        ColorChoice::Simple(_) => {
            rsx! {
                ToggleDiv {
                    is_selected: selected_color() == format!("{color_str}-foreground"),
                    onclick: move |_| {
                        *selected_color.write() = format!("{color_str}-foreground");
                    },
                    Icon { icon: Icons::Colorize }
                }
                div { style: "background-color: var(--{color_str}); flex-grow: 1; display: flex; align-items: center; justify-content: center;",
                    p {
                        color: "white",
                        text_shadow: "-1px -1px 0 #000, 1px -1px 0 #000, -1px 1px 0 #000, 1px 1px 0 #000",
                        "{color_str}"
                    }
                }
            }
        }
        ColorChoice::Duo(_, _) => {
            let bg = if &*color_str.read() == "background" {
                "background".to_string()
            } else {
                format!("{color_str}")
            };

            let text = if &*color_str.read() == "background" {
                "foreground".to_string()
            } else {
                format!("{color_str}-foreground")
            };

            let is_selected = if &*color_str.read() == "background" {
                selected_color() == "foreground"
            } else {
                selected_color() == format!("{color_str}-foreground")
            };

            rsx! {
                ToggleDiv {
                    is_selected,
                    onclick: move |_| {
                        if &*color_str.read() == "background" {
                            log::debug!("there");
                            *selected_color.write() = "foreground".to_string();
                        } else {
                            *selected_color.write() = format!("{color_str}-foreground");
                        }
                    },
                    Icon { icon: Icons::FlipToFront }
                }
                p { style: r#"
                        background-color: var(--{bg});
                        color: var(--{text});
                        text-align: center;
                        font-size: 0.875rem;
                        font-weight: 700;
                        flex-grow: 1;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    "#,
                    "{color_str}"
                }
                ToggleDiv {
                    is_selected: *selected_color.read() == *color_str.read(),
                    onclick: move |_| {
                        selected_color.write().clone_from(&*color_str.read());
                    },
                    Icon { icon: Icons::FlipToBack }
                }
            }
        }
    };

    rsx! {
        div { class: "theme-picker-color-selector",
            {content}
        }
    }
}

#[component]
fn ToggleDiv(is_selected: bool, onclick: EventHandler<MouseEvent>, children: Element) -> Element {
    let onclick = move |event| {
        onclick.call(event);

        // This is to trigger the click event on the color picker input
        // Will only work with web
        spawn(async move {
            let _eval = document::eval(
                r#"
                const input = document.getElementById("color-picker-input");
                if (input != null) {
                    input.click();
                }
                "#,
            );
        });
    };

    rsx! {
        div {
            role: "button",
            class: "theme-toggle-div",
            "data-selected": is_selected,
            onclick,
            {children}
        }
    }
}

#[component]
fn RadiusSelector() -> Element {
    let mut theme_manager = use_context::<Signal<ThemeManager>>();

    let current_theme = theme_manager.read().current_theme;

    rsx! {
        div { id: "radius-selector", class: "w-full",
            p { style: "font-size: 0.875rem; font-weight: 700;", "Radius" }
            Input {
                "data-size": "sm",
                r#type: "text",
                default_value: theme_manager.read().themes[current_theme].radius.to_style(),
                onchange: move |event: FormEvent| {
                    let value = event.data().value();
                    theme_manager.write().themes[current_theme].radius = RadiusCss(value);
                },
            }
        }
    }
}

#[component]
fn MiniPicker() -> Element {
    let mut theme_manager = use_context::<Signal<ThemeManager>>();

    let current_theme = theme_manager.read().current_theme;

    rsx! {
        div { style: "display: flex; flex-direction: row; padding: 0.5rem; align-items: center;",
            SidePanelTrigger { class: "theme-minipicker-trigger",
                Icon { icon: Icons::Palette }
            }
            LightSwitch {
                class: "theme-minipicker-lightswitch",
                onclick: move |_| {
                    if current_theme == 0 {
                        theme_manager.write().current_theme = 1
                    } else {
                        theme_manager.write().current_theme = 0
                    }
                },
            }
        }
    }
}

#[component]
fn ButtonExport() -> Element {
    rsx! {
        Modal {
            ModalTrigger { style: "width: 100%; align-text: center;", "Export Theme" }
            ModalBackground {}
            ModalContent {
                ModalClose {}
                h6 { class: "h6", "Theme" }
                p { style: "font-size: 0.875rem; font-widht: 700; color: color-mix(in oklab, var(--foreground) 50%, transparent); padding-bottom: 1rem;",
                    "Copy and paste this in your project's CSS file."
                }
                ThemeExport {}
            }
        }
    }
}

#[component]
fn ThemeExport() -> Element {
    let theme_manager = use_context::<Signal<ThemeManager>>();

    rsx! {
        Scrollable {
            style: "max-height: 20rem; border: none; background-color: var(--foreground);",
            pre { style: "background-color: var(--foreground); color: var(--background); padding-left: 1rem; padding-right: 3rem; padding-top: 0.5rem; padding-bottom: 0.5rem; border-radius: var(--radius)",
                code { style: "font-size: 0.875rem;", "{theme_manager.read().export_to_css()}" }
            }
        }
    }
}
