use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct MarkdownProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The markdown content to render
    /// Example: "# Hello World"
    /// Default: ""
    #[props(into, default = String::new())]
    content: String,
}

/// Uses `dangerous_inner_html` to render markdown content as HTML
#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = string_to_html(props.content.clone());

    rsx! {
        div {
            class: "prose dark:prose-invert max-w-none",
            dangerous_inner_html: "{content}",
            ..props.attributes
        }
    }
}

/// Convert a markdown string to HTML
/// Uses pulldown-cmark crate
/// Supports tables, footnotes, strikethrough, tasklists, and smart punctuation
/// # Example
/// ```
/// let html = string_to_html("# Hello World");
/// assert_eq!(html, "<h1>Hello World</h1>");
/// ```
fn string_to_html(content: String) -> String {
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
