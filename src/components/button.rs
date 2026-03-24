use dioxus::prelude::*;



#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    /// Additional attributes to apply to the element
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The click event handler
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    /// The double click event handler
    #[props(optional)]
    ondoubleclick: EventHandler<MouseEvent>,
    /// The mouse down event handler
    #[props(optional)]
    onmousedown: EventHandler<MouseEvent>,
    /// The mouse up event handler
    #[props(optional)]
    onmouseup: EventHandler<MouseEvent>,

    /// Remove default CSS classes
    #[props(default = false)]
    noclasses: bool,

    /// Show loading state: spinner + "Loading..." text, button disabled
    #[props(default = false)]
    loading: bool,

    /// The children element
    children: Element,
}

#[component]
pub fn Button(mut props: ButtonProps) -> Element {
    let default_classes = "button";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let loading = props.loading;
    let onclick = move |event| {
        if !loading {
            props.onclick.call(event);
        }
    };
    let ondoubleclick = move |event| {
        if !loading {
            props.ondoubleclick.call(event);
        }
    };
    let onmousedown = move |event| {
        if !loading {
            props.onmousedown.call(event);
        }
    };
    let onmouseup = move |event| {
        if !loading {
            props.onmouseup.call(event);
        }
    };

    rsx! {
        button {
            disabled: loading,
            onclick,
            ondoubleclick,
            onmousedown,
            onmouseup,
            ..props.attributes,
            if loading {
                style { "@keyframes btn-spin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}" }
                span {
                    class: "inline-flex items-center gap-2 opacity-70 pointer-events-none",
                    span {
                        style: "font-family: 'Material Symbols Rounded'; font-size: 1.2em; line-height: 1; animation: btn-spin 1s linear infinite; display: inline-block;",
                        "progress_activity"
                    }
                    "Loading..."
                }
            } else {
                {props.children}
            }
        }
    }
}
