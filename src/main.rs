#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_components_bin::atom::button::*;
use dioxus_components_bin::atom::checkbox::*;
use dioxus_components_bin::atom::formrange::*;
use dioxus_components_bin::atom::icon::style::IconSvg;
use dioxus_components_bin::atom::icon::*;
use dioxus_components_bin::atom::input::*;
use dioxus_components_bin::atom::label::*;
use dioxus_components_bin::atom::textarea::*;
use dioxus_components_bin::atom::toggle::*;
use dioxus_components_bin::composite::formlist::*;
use dioxus_components_bin::composite::lightswitch::*;
use dioxus_components_bin::composite::radiogroup::*;
use dioxus_components_bin::composite::select::*;
use InputType::*;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::WARN)
            .build(),
    );
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new(LightSwitchSignal("".to_string())));

    let light_switch_context = use_context::<Signal<LightSwitchSignal>>();
    let dark = &light_switch_context.read().0;
    rsx!(
        body { class: "{dark} bg-background",
            div { LightSwitch {} }
            // div { TestCheckbox {} }
            // div { TestFormRange{}}
            // div { TestRadio {} }
            // div { TestIcon {} }
            // div { TestInput {} }
            // div { TestTextArea {}}
            // div { TestToggle {} }
            // div { TestLightSwitch {} }
            div { TestRadioGroup {} }
        }
    )
}

// Atoms
fn TestButton() -> Element {
    let keyboard_closure = move |event: FormEvent| log::debug!("{}", event.value());
    rsx!(
        div { class: "",
            Button { color: Unset, "Unset" }
            Button { color: Primary, "Primary" }
            Button { color: Secondary, "Secondary" }
            Button { color: Accent, "Accent" }
            Button { variant: Outline(Primary), "Outline" }
            Button { variant: Outline(Secondary), "Outline Secondary" }
            Button { variant: Outline(Accent), "Outline Accent" }
            Button { variant: Ghost(Primary), "Ghost" }
            Button { variant: Ghost(Secondary), "Ghost Secondary" }
            Button { variant: Ghost(Accent), "Ghost Accent" }
            Button { size: Xs, variant: Ghost(Primary), "Ghost Primary Xs" }
        }
        div { class: "",
            Button { size: Sm, "Sm" }
            Button { "Default" }
            Button { size: Lg, "Lg" }
            Button { size: Xl, "Xl" }
        }
        div { class: "", TextArea { oninput: keyboard_closure } }
    )
}

fn TestCheckbox() -> Element {
    rsx!(
        div { class: "flex gap-2",
            div { class: "",
                Checkbox { name: "checkbox", value: "checkbox", color: Primary, "Primary" }
            }
            div { class: "",
                Checkbox { name: "checkbox", value: "checkbox", color: Secondary, "Secondary" }
            }
            div { class: "",
                Checkbox { name: "checkbox", value: "checkbox", color: Accent, "Accent" }
            }
            // div { class: "",
            //     Checkbox { name: "checkbox", value: "checkbox", color: Accent, "Accent" }
            // }
            div { class: "",
                Checkbox {
                    name: "checkbox",
                    value: "checkbox",
                    color: Primary,
                    checked: true,
                    disabled: true,
                    "Checkbox"
                }
            }
            div { class: "",
                Checkbox { name: "checkbox", value: "checkbox", color: Accent, disabled: true, "Checkbox" }
            }
        }
    )
}

fn TestFormRange() -> Element {
    rsx!(
        div { class: "flex",
            div { class: "",
                FormRange { name: "range", min: 0, max: 100, step: 1 }
                FormRange { name: "range", min: 0, max: 100, step: 1, disabled: true }
            }
        }
    )
}

fn TestIcon() -> Element {
    rsx!(
        div { class: "flex gap-4",
            // TODO
            // div { class: "size-8", Icon { svg: IconSvg::CircleInnerCircle, color: Default } }
            div { class: "size-8", Icon { svg: IconSvg::CircleInnerCircle, color: Primary } }
            div { class: "size-8", Icon { svg: IconSvg::CircleInnerCircle, color: Secondary } }
            div { class: "size-8", Icon { svg: IconSvg::CircleInnerCircle, color: Accent } }
            div { class: "size-8", Icon { svg: IconSvg::HollowCircle, color: Primary } }
            div { class: "size-8", Icon { svg: IconSvg::HollowCircle, color: Secondary } }
            div { class: "size-8", Icon { svg: IconSvg::HollowCircle, color: Accent } }
        }
    )
}

fn TestInput() -> Element {
    rsx!(
        div { class: "flex gap-4",
            div { class: "",
                Input { r#type: Text, name: "text", placeholder: "Text" }
                Input { r#type: Text, name: "text", placeholder: "Text", disabled: true }
            }
            div { class: "",
                Input { r#type: Email, name: "email", placeholder: "Email" }
                Input { r#type: Email, name: "email", placeholder: "Email", disabled: true }
            }
            div { class: "",
                Input { r#type: Number, name: "number", placeholder: "Number" }
                Input { r#type: Number, name: "number", placeholder: "Number", disabled: true }
            }
            div { class: "",
                Input { r#type: Date, name: "date", placeholder: "Date" }
                Input { r#type: Date, name: "date", placeholder: "Date", disabled: true }
            }
        }
    )
}

fn TestTextArea() -> Element {
    rsx!(
        div { class: "flex gap-4",
            div { class: "",
                TextArea { name: "textarea", placeholder: "TextArea" }
                TextArea { name: "textarea", placeholder: "TextArea", disabled: true }
            }
        }
    )
}

fn TestToggle() -> Element {
    rsx!(
        div { class: "flex gap-4",
            div { class: "",
                Toggle { name: "toggle", value: "toggle", checked: true, color: Primary, "Primary" }
                Toggle { name: "toggle", value: "toggle", checked: true, color: Secondary, "Secondary" }
                Toggle { name: "toggle", value: "toggle", checked: true, color: Accent, "Accent" }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: Primary,
                    disabled: true,
                    "Disabled"
                }
                Toggle { name: "toggle", value: "toggle", checked: true, color: Primary, size: Sm, "Sm" }
                Toggle { name: "toggle", value: "toggle", checked: true, color: Primary, size: Lg, "Lg" }
            }
        }
    )
}

fn TestForm() -> Element {
    let mut values = use_signal(HashMap::new);
    rsx!(
        div {
            "Test"
            form {
                class: "border border-black w-96",
                method: "POST",
                id: "option-form",
                onsubmit: move |event| {
                    log::debug!("Form values {:#?}", values());
                    log::debug!("Valid :{}", event.valid());
                    values.set(event.values());
                },
                // oninput: move |event| {
                //     values.set(event.values());
                // },
                RadioGroup { name: "gender", default_value: "male",
                    Label { r#for: "gender", "Choose birth gender" }
                    RadioItem { value: "male", name: "gender", required: true, "Male" }
                    RadioItem { value: "female", name: "gender", "Female" }
                    RadioItem { value: "other", name: "gender", "Other" }
                    RadioItem { value: "disabled", name: "gender", disabled: true, "Disabled" }
                }
                div { class: "flex flex-col",
                    Label { r#for: "username", "Your username" }
                    Input { r#type: Text, name: "username", placeholder: "username" }
                }
                div {
                    Label { r#for: "email", "Your email" }
                    Input { r#type: Email, name: "email", placeholder: "email" }
                }
                div {
                    Label { r#for: "age", "Your age" }
                    Input { r#type: Number, name: "age", placeholder: "age", min: 18, max: 100 }
                }
                div {
                    Label { r#for: "date", "Your Piscine date" }
                    Input { r#type: Date, name: "date", placeholder: "Piscine date" }
                }
                div {
                    Label { r#for: "message", "Send us a message" }
                    TextArea { name: "message", placeholder: "Your message..." }
                }
                div {
                    Label { r#for: "animal", "Select an animal" }
                    SelectGroup { name: "animal",
                        SelectPlaceholder { "Select an animal" }
                        SelectLabel { label: "Domestic",
                            SelectItem { value: "dog", "Dog" }
                            SelectItem { value: "cat", "Cat" }
                            SelectItem { value: "hamster", "Hamster" }
                            SelectItem { value: "none", disabled: true, "None" }
                        }
                        SelectLabel { label: "Wild", disabled: true,
                            SelectItem { value: "lion", "Lion" }
                            SelectItem { value: "tiger", "Tiger" }
                            SelectItem { value: "bear", "Bear" }
                        }
                    }
                }
                div {
                    Label { r#for: "activities", "Select your fav activities" }
                    Checkbox { name: "activities", value: "reading", "Reading" }
                    Checkbox { name: "activities", value: "coding", checked: true, "Coding" }
                    Checkbox { name: "activities", value: "writing", "Writing" }
                    Checkbox { name: "activities", value: "swimming", "Swimming" }
                    Checkbox { name: "activities", value: "football", "Football" }
                    Checkbox { name: "activities", value: "none", disabled: true,
                        Label { "None" }
                    }
                }
                div {
                    Label { r#for: "rate", "Rate us" }
                    FormRange { name: "rate", min: 0, max: 100, step: 1 }
                }
                div {
                    Toggle { name: "newsletter", value: "newsletter", checked: true,
                        Label { r#for: "newsletter", "Subscribe to newsletter" }
                    }
                }
                div {
                    Toggle { name: "toggle1", value: "toggle1", checked: true, color: Secondary,
                        Label { "toggle1" }
                    }
                }
                div {
                    Toggle {
                        name: "toggle2",
                        value: "toggle2",
                        checked: true,
                        color: Accent,
                        size: Sm,
                        Label { "toggle2" }
                    }
                }
                div {
                    Toggle { name: "toggle3", value: "toggle3", disabled: true, size: Lg,
                        Label { "toggle3" }
                    }
                }
                div {
                    Toggle { name: "cookie", value: "cookie", checked: false,
                        Label { "Use our cookie" }
                    }
                }
                div {
                    Checkbox { name: "terms", value: "yes", required: false,
                        Label { "Accept terms and conditions" }
                    }
                }
                div {
                    Button { variant: Ghost(Primary), size: Sm, r#type: "submit", "Submit" }
                }
            }
        }
        div { "Values: {values:#?}" }
    )
}

// Composites

fn TestListForm() -> Element {
    let mut values = use_signal(HashMap::new);
    // let input1 = rsx!(
    //     SelectGroup { name: "animal",
    //         SelectPlaceholder { "Select an animal" }
    //         SelectLabel { label: "Domestic",
    //             SelectItem { value: "dog", "Dog" }
    //             SelectItem { value: "cat", "Cat" }
    //             SelectItem { value: "hamster", "Hamster" }
    //             SelectItem { value: "none", disabled: true, "None" }
    //         }
    //         SelectLabel { label: "Wild", disabled: true,
    //             SelectItem { value: "lion", "Lion" }
    //             SelectItem { value: "tiger", "Tiger" }
    //             SelectItem { value: "bear", "Bear" }
    //         }
    //     }
    //     SelectGroup { name: "number-animal",
    //         SelectPlaceholder { "Select number of animals" }
    //         SelectLabel { label: "1-10",
    //             SelectItem { value: "1", "1" }
    //             SelectItem { value: "2", "2" }
    //             SelectItem { value: "3", "3" }
    //             SelectItem { value: "4", "4" }
    //             SelectItem { value: "5", "5" }
    //             SelectItem { value: "6", "6" }
    //             SelectItem { value: "7", "7" }
    //             SelectItem { value: "8", "8" }
    //             SelectItem { value: "9", "9" }
    //             SelectItem { value: "10", "10" }
    //         }
    //     }
    //     Label { r#for: "name-animal", "Type animal name" }
    //     Input { r#type: Text, name: "name-animal", placeholder: "Animal name" }
    // );

    // TODO Should do this in a Macro
    let mut group = Vec::<Element>::new();
    for i in 0..5 {
        group.push(rsx!(
            SelectGroup { name: "animal-{i}",
                SelectPlaceholder { "Select an animal" }
                SelectLabel { label: "Domestic",
                    SelectItem { value: "dog", "Dog" }
                    SelectItem { value: "cat", "Cat" }
                    SelectItem { value: "hamster", "Hamster" }
                    SelectItem { value: "none", disabled: true, "None" }
                }
                SelectLabel { label: "Wild", disabled: true,
                    SelectItem { value: "lion", "Lion" }
                    SelectItem { value: "tiger", "Tiger" }
                    SelectItem { value: "bear", "Bear" }
                }
            }
            SelectGroup { name: "number-animal-{i}",
                SelectPlaceholder { "Select number of animals" }
                SelectLabel { label: "1-10",
                    SelectItem { value: "1", "1" }
                    SelectItem { value: "2", "2" }
                    SelectItem { value: "3", "3" }
                    SelectItem { value: "4", "4" }
                    SelectItem { value: "5", "5" }
                    SelectItem { value: "6", "6" }
                    SelectItem { value: "7", "7" }
                    SelectItem { value: "8", "8" }
                    SelectItem { value: "9", "9" }
                    SelectItem { value: "10", "10" }
                }
            }
            Label { r#for: "name-animal-{i}", "Type animal name" }
            Input { r#type: Text, name: "name-animal-{i}", placeholder: "Animal name" }
        ));
    }

    rsx!(
        div {
            "TestListForm"
            div { class: "border border-black w-96",
                form {
                    // method: "POST",
                    id: "id-group",
                    onsubmit: move |event| {
                        log::debug!("Form values {:#?}", values());
                        log::debug!("Valid :{}", event.valid());
                        values.set(event.values());
                    },
                    FormList { group_vec: group }
                    Button { r#type: "submit", "Submit" }
                }
            }
        }
    )
}

fn TestLightSwitch() -> Element {
    rsx!(
        div { class: "flex gap-4",
            div { class: "", LightSwitch {} }
        }
    )
}

fn TestRadioGroup() -> Element {
    rsx!(
        RadioGroup { name: "gender", default_value: "male",
            Label { r#for: "gender", "Choose birth gender" }
            RadioItem { value: "male", name: "gender", required: true, "Male" }
            RadioItem { value: "female", name: "gender", "Female" }
            RadioItem { value: "other", name: "gender", "Other" }
            RadioItem { value: "disabled", name: "gender", disabled: true, "Disabled" }
        }
    )
}

fn TestSelect() -> Element {
    rsx!(
        div { class: "flex w-96 gap-4",
            SelectGroup { name: "animal",
                SelectPlaceholder { "Select an animal" }
                SelectLabel { label: "Domestic",
                    SelectItem { value: "dog", "Dog" }
                    SelectItem { value: "cat", "Cat" }
                    SelectItem { value: "hamster", "Hamster" }
                    SelectItem { value: "none", disabled: true, "None" }
                }
                SelectLabel { label: "Wild", disabled: true,
                    SelectItem { value: "lion", "Lion" }
                    SelectItem { value: "tiger", "Tiger" }
                    SelectItem { value: "bear", "Bear" }
                }
            }
        }
    )
}
