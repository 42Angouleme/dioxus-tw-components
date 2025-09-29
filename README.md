# Dioxus Components [![Main CI](https://github.com/42Angouleme/dioxus-tw-components/actions/workflows/mail.yml/badge.svg)](https://github.com/42Angouleme/dioxus-tw-components/actions/workflows/mail.yml)

A simple but highly customizable and efficient cross-platform components library for Dioxus 0.7.

## List of available components

All components are compatible with dioxus Web, Desktop and Mobile.

Here's a non exhaustive list of all currently available components:

<details>
    <summary>
        Components
    </summary>
    <table>
        <tr><td>Button</td></tr>
        <tr><td>Button Group</td></tr>
        <tr><td>Icon</td></tr>
        <tr><td>Placeholder</td></tr>
        <tr><td>Separator</td></tr>
        <tr><td>Accordion</td></tr>
        <tr><td>Callout</td></tr>
        <tr><td>Carousel</td></tr>
        <tr><td>Dropdown</td></tr>
        <tr><td>Hovercard</td></tr>
        <tr><td>LightSwitch</td></tr>
        <tr><td>Markdown</td></tr>
        <tr><td>Modal</td></tr>
        <tr><td>Navbar</td></tr>
        <tr><td>ProgressBar</td></tr>
        <tr><td>Scrollable</td></tr>
        <tr><td>SidePanel</td></tr>
        <tr><td>SortTable</td></tr>
        <tr><td>Table</td></tr>
        <tr><td>Tabs</td></tr>
        <tr><td>Toast</td></tr>
        <tr><td>Checkbox</td></tr>
        <tr><td>FormList</td></tr>
        <tr><td>Input</td></tr>
        <tr><td>Radio</td></tr>
        <tr><td>Select</td></tr>
        <tr><td>Slider</td></tr>
        <tr><td>TextArea</td></tr>
        <tr><td>Toggle</td></tr>
    </table>
</details>

## Docsite

Dioxus Components offers a docsite to showcase the components and experiment with them.
Here is the [docsite (live)](https://42angouleme.github.io/dioxus-tw-components-docsite) showcase _a la Storybook_, which also comes with a live theme customizer and exporter.
While most of the time the showcase will be your first choice, you might want to manipulate the docsite source code, which you'll find [here](https://github.com/42Angouleme/dioxus-tw-components-docsite).
Additionally, you can use it to export custom themes to embed in your own projects.

## Getting started

To add this library to your Dioxus project, you can just run the following:
```bash
cargo add dioxus-tw-components
```

### Boostrap the library

To work properly, the library needs to be launched at the beginning of your application:

```rust
use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Launches Dioxus Components. Some components may not work without this.
        Bootstrap {}

        // Rest of your application
    }
}
```

### Input CSS

Dioxus Components uses special CSS variable names to style properly. You may add them to your css files:
<details>
    <summary>
        Example variable CSS file
    </summary>

```css
:root {
    --background: /* Color */;
    --foreground: /* Color */;
    --primary: /* Color */;
    --primary-foreground: /* Color */;
    --secondary: /* Color */;
    --secondary-foreground: /* Color */;
    --accent: /* Color */;
    --accent-foreground: /* Color */;
    --muted: /* Color */;
    --muted-foreground: /* Color */;
    --destructive: /* Color */;
    --destructive-foreground: /* Color */;
    --success: /* Color */;
    --success-foreground: /* Color */;
    --border: /* Color */;
    --input: /* Color */;
    --popover: /* Color */;
    --shadow: /* Shadow data */;
    --radius: /* Radius */;
}
.dark {
    --background: /* Color */;
    --foreground: /* Color */;
    --primary: /* Color */;
    --primary-foreground: /* Color */;
    --secondary: /* Color */;
    --secondary-foreground: /* Color */;
    --accent: /* Color */;
    --accent-foreground: /* Color */;
    --border: /* Color */;
    --input: /* Color */;
    --popover: /* Color */;
    --shadow: /* Shadow data */;
}
```

</details>

## Disclaimer

This repository contains an experimental component library for Dioxus, derived from our internal work and needs.
We are sharing it with the community as-is, so you can explore, adapt, and build upon our work.

Please note:

* Not production ready:
    * This library is provided for experimental and educational purposes only. It is not designed for production use.

* Community-driven evolution:
    * We are offering it to the community as a starting point. Feel free to fork, modify, and enhance it in your own repositories.

* Limited maintenance commitment:
    * We commit to reviewing any pull requests related to bugs, improvements, and component additions until July 2025.
After that date, we are not guaranteeing to manage or support any future developments in this library.

* No major development planned:
    * We do not intend to invest significant further development in this project.

* Respecting the official ecosystem:
    * Our goal is not to compete with the upcoming official Dioxus component library. We fully support the evolution of the Dioxus ecosystem and see our contribution as complementary and a helping hand.

We hope that this initiative serves as a useful resource and inspiration for your projects!

## License

This project is licensed under either the [MIT license](./LICENSE-MIT) or the [Apache-2 License](./LICENSE-APACHE).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Dioxus Components by you shall be licensed as MIT or Apache-2 without any additional terms or conditions.
