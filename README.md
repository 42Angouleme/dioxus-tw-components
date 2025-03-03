# Dioxus Tailwind Components [![Main CI](https://github.com/42Angouleme/dioxus-tw-components/actions/workflows/mail.yml/badge.svg)](https://github.com/42Angouleme/dioxus-tw-components/actions/workflows/mail.yml)

A simple but highly customizable and efficient cross-platform components library for Dioxus 0.6 based on TailwindCSS 4.

## List of available components

Here's a non exhaustive list of all currently available components. They are divided in 3 categories based on their complexity:

<details>
    <summary>
        Atoms
    </summary>
    <table>
        <tr><td>Button</td></tr>
        <tr><td>Button Group</td></tr>
        <tr><td>Icon</td></tr>
        <tr><td>Placeholder</td></tr>
        <tr><td>Separator</td></tr>
        <tr><td>Spacer</td></tr>
    </table>
</details>

<details>
    <summary>
        Molecules
    </summary>
    <table>
        <tr><td>Accordion</td></tr>
        <tr><td>Breadcumb</td></tr>
        <tr><td>Carousel</td></tr>
        <tr><td>Dropdown</td></tr>
        <tr><td>Hovercard</td></tr>
        <tr><td>LightSwitch</td></tr>
        <tr><td>Modal</td></tr>
        <tr><td>Navbar</td></tr>
        <tr><td>ProgressBar</td></tr>
        <tr><td>Scrollable</td></tr>
        <tr><td>SidePanel</td></tr>
        <tr><td>SortTable</td></tr>
        <tr><td>Table</td></tr>
        <tr><td>Tabs</td></tr>
        <tr><td>Toast</td></tr>
    </table>
</details>

<details>
    <summary>
        Organisms
    </summary>
    <table>
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

## Getting started

### Public crate

To add this library to your Dioxus project, you can just run the following:
```bash
cargo add dioxus-tw-components
```

You then need to tell the Tailwind compiler where to find the Dioxus Tailwind Components. You can do so by adding this line to your tailwind configuration file:
```js
// {process.env.HOME} is usually where the .cargo directory is. This should be replaced by the actual path if yours is somewhere else.
`${process.env.HOME}/.cargo/registry/src/**/dioxus-tw-components-[version or *]/src/**/*.{rs,html,css}`
```

### Local crate

If you want to use this library as a local crate, you can add this to your `Cargo.toml` file:
```ini
dioxus-tw-components = { path = "path/to/dioxus-tw-components" }
```

You then need to tell the Tailwind compiler where to find the Dioxus Tailwind Components. You can do so by adding this line to your tailwind configuration file:
```js
"path/to/dioxus-tw-components/src/**/*.{rs,html,css}"
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
        // Launches Dioxus Tailwind Components. Some components may not work without this.
        DioxusTwComponentsBootstrap {}

        // Rest of your application
    }
}
```

### Input CSS

Dioxus Tailwind Components uses special CSS variable names to style properly. You may add them to your `input.css` file before feeding it to tailwind:
<details>
    <summary>
        Example `input.css` file
    </summary>

```css
@import 'tailwindcss';

@layer base {
    :root {
        --background: /* HSL color */;
        --foreground: /* HSL color */;
        --primary: /* HSL color */;
        --primary-foreground: /* HSL color */;
        --secondary: /* HSL color */;
        --secondary-foreground: /* HSL color */;
        --accent: /* HSL color */;
        --accent-foreground: /* HSL color */;
        --muted: /* HSL color */;
        --muted-foreground: /* HSL color */;
        --destructive: /* HSL color */;
        --destructive-foreground: /* HSL color */;
        --success: /* HSL color */;
        --success-foreground: /* HSL color */;
        --border: /* HSL color */;
        --input: /* HSL color */;
        --ring: /* HSL color */;
        --global-shadow: /* Shadow data */;
        --global-radius: /* Radius */;
    }
    .dark {
        --background: /* HSL color */;
        --foreground: /* HSL color */;
        --primary: /* HSL color */;
        --primary-foreground: /* HSL color */;
        --secondary: /* HSL color */;
        --secondary-foreground: /* HSL color */;
        --accent: /* HSL color */;
        --accent-foreground: /* HSL color */;
        --border: /* HSL color */;
        --input: /* HSL color */;
        --ring: /* HSL color */;
        --global-shadow: /* Shadow data */;
    }
    .h1 {
        @apply text-4xl font-extrabold md:text-5xl;
    }
    .h2 {
        @apply text-2xl font-bold md:text-4xl;
    }
    .h3 {
        @apply text-2xl font-semibold md:text-3xl;
    }
    .h4 {
        @apply text-xl font-semibold md:text-2xl;
    }
    .h5 {
        @apply text-lg font-semibold md:text-xl;
    }
    .h6 {
        @apply text-base font-semibold md:text-lg;
    }
    .paragraph {
        @apply font-normal text-foreground;
    }
    .span {
        @apply font-normal text-foreground;
    }
    .anchor {
        @apply text-foreground/70 hover:text-foreground transition-colors cursor-pointer;
    }
}
```

</details>

## Docsite

Dioxus Tailwind Components offers a [docsite](https://42angouleme.github.io/dioxus-tw-components-docsite) to showcase the components and experiment with them.
Additionally, you can use it to export custom themes to embed in your own projects.

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

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Dioxus Tailwind Components by you shall be licensed as MIT or Apache-2 without any additional terms or conditions.
