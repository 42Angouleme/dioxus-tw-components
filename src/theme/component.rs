use crate::{prelude::*, theme::*};
use dioxus::prelude::*;

#[component]
pub fn ThemePicker() -> Element {
    rsx! {
        SidePanel {
            MiniPicker {}
            SidePanelBackground { class: "z-998" }
            SidePanelContent {
                class: "h-screen min-w-0 w-full sm:w-120 p-0 sm:px-8 z-999",
                side: Side::Right,
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

    let oninput = move |event: FormEvent| {
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
        div { class: "flex flex-col items-start overflow-auto h-full mt-20 py-auto px-2 sm:px-4 space-y-2",
            Input {
                role: "button",
                id: "color-picker-input",
                r#type: "color",
                class: "hidden",
                oninput,
            }
            for (str , color) in theme_manager.read().themes[current_theme].colors.iter() {
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
    color_str: ReadOnlySignal<String>,
    color: ColorChoice,
    mut selected_color: Signal<String>,
) -> Element {
    let content: Element = match color {
        ColorChoice::Simple(_) => {
            rsx! {
                ToggleDiv {
                    is_selected: selected_color() == format!("{}-foreground", color_str),
                    onclick: move |_| {
                        *selected_color.write() = format!("{}-foreground", color_str);
                    },
                    Icon { icon: Icons::Colorize }
                }
                div { class: "bg-{color_str} grow flex items-center justify-center",
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
                "bg-background".to_string()
            } else {
                format!("bg-{}", color_str)
            };

            let text = if &*color_str.read() == "background" {
                "text-foreground".to_string()
            } else {
                format!("text-{}-foreground", color_str)
            };

            let is_selected = if &*color_str.read() == "background" {
                selected_color() == "foreground"
            } else {
                selected_color() == format!("{}-foreground", color_str)
            };

            rsx! {
                ToggleDiv {
                    is_selected,
                    onclick: move |_| {
                        if &*color_str.read() == "background" {
                            log::debug!("there");
                            *selected_color.write() = "foreground".to_string();
                        } else {
                            *selected_color.write() = format!("{}-foreground", color_str);
                        }
                    },
                    Icon { icon: Icons::FlipToFront }
                }
                p { class: "{bg} {text} text-center text-sm font-bold grow flex items-center justify-center",
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
        div { class: "w-full rounded-global-radius p-1 space-x-1 flex bg-backgroud text-foreground font-bold text-sm border border-border",
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
            class: "group rounded-global-radius p-1 transition-all data-[selected=true]:bg-accent active:bg-foreground/45",
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
            p { class: "text-sm font-medium", "Radius" }
            Input {
                size: Size::Sm,
                r#type: "text",
                value: theme_manager.read().themes[current_theme].radius.to_style(),
                oninput: move |event: FormEvent| {
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
        div { class: "flex flex-row p-2 items-center space-x-4",
            SidePanelTrigger { class: "border-none shadow-none bg-inherit cursor-pointer p-1 rounded-global-radius hover:bg-foreground/40 active:bg-foreground/60",
                Icon { icon: Icons::Palette }
            }
            LightSwitch {
                class: "cursor-pointer p-1 rounded-global-radius hover:bg-foreground/40 active:bg-foreground/60",
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
            ModalTrigger { class: "w-full text-center", "Export Theme" }
            ModalBackground {}
            ModalContent {
                ModalClose {}
                h6 { class: "h6", "Theme" }
                p { class: "text-sm font-medium text-foreground/50 pb-4",
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
            orientation: Orientation::Vertical,
            class: "max-h-80 border-none bg-foreground",
            pre { class: "bg-foreground text-background pl-4 pr-12 py-2 rounded-global-radius",
                code { class: "text-sm", "{theme_manager.read().export_to_css()}" }
            }
        }
    }
}
