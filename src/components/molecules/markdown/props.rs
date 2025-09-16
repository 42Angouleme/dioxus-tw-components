use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct MarkdownProps {
    /// The markdown content to render
    /// Example: "# Hello World"
    /// Default: ""
    #[props(into, default = String::new())]
    pub content: String,
}

/// Use `dangerous_inner_html` to render markdown content as HTML
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = stringToHTML(props.content.clone());

    rsx! {
        div {
            class: "prose dark:prose-invert max-w-none",
            dangerous_inner_html: "{content}"
        }
    }
}

/// Convert a markdown string to HTML
/// Uses pulldown-cmark crate
/// Supports tables, footnotes, strikethrough, tasklists, and smart punctuation
/// # Example
/// ```
/// let html = stringToHTML("# Hello World");
/// assert_eq!(html, "<h1>Hello World</h1>");
/// ```
fn stringToHTML(content: String) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}
