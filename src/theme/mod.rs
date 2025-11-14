use std::{collections::HashMap, error::Error};

pub mod component;

pub use component::ThemePicker;

// TODO
// Theme importer
// Add Color like Primary, Secondary,...
// Support for RGB colors

#[derive(Debug, Clone)]
pub struct ThemeManager {
    themes: Vec<Theme>,
    pub current_theme: usize,
}

impl std::default::Default for ThemeManager {
    fn default() -> Self {
        Self {
            themes: vec![Theme::default(), Theme::dark()],
            current_theme: 1,
        }
    }
}

impl ToStyle for ThemeManager {
    fn to_style(&self) -> String {
        self.themes[self.current_theme].to_style()
    }
}

impl ExportToCss for ThemeManager {
    fn export_to_css(&self) -> String {
        let mut css = String::from("@layer base {\n\n");

        for theme in self.themes.iter() {
            css.push_str(&theme.export_to_css());
            css.push('\n');
        }

        css.push('}');
        css
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    name: String,
    colors: HashMap<String, ColorChoice>,
    radius: RadiusCss,
}

impl ToStyle for Theme {
    fn to_style(&self) -> String {
        let mut style = String::new();

        for (key, color_choice) in self.colors.iter() {
            match color_choice {
                ColorChoice::Simple(color) => {
                    style.push_str(&format!(" --{}: {};", key, color.to_style()));
                }
                ColorChoice::Duo(background, foreground) => {
                    style.push_str(&format!(" --{}: {};", key, background.to_style()));
                    if key != "background" {
                        style.push_str(&format!(
                            " --{}-foreground: {};",
                            key,
                            foreground.to_style()
                        ));
                    } else {
                        style.push_str(&format!(" --foreground: {};", foreground.to_style()));
                    }
                }
            }
        }

        style.push_str(&format!(" --radius: {};\n", self.radius.to_style()));

        style
    }
}

impl ExportToCss for Theme {
    fn export_to_css(&self) -> String {
        let first_char = if self.name == "root" { ':' } else { '.' };

        let mut css = format!(" {}{} {{\n", first_char, self.name);

        for (key, color_choice) in self.colors.iter() {
            match color_choice {
                ColorChoice::Simple(color) => {
                    css.push_str(&format!("  --{}: {};\n", key, color.to_style()));
                }
                ColorChoice::Duo(background, foreground) => {
                    css.push_str(&format!("  --{}: {};\n", key, background.to_style()));
                    if key != "background" {
                        css.push_str(&format!(
                            "  --{}-foreground: {};\n",
                            key,
                            foreground.to_style()
                        ));
                    } else {
                        css.push_str(&format!("  --foreground: {};\n", foreground.to_style()));
                    }
                }
            }
        }

        css.push_str(&format!("  --radius: {};\n", self.radius.to_style()));

        css.push_str(" }\n");

        css
    }
}

impl std::default::Default for Theme {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert(
            "background".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 96.08,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 20.0,
                },
            ),
        );

        colors.insert(
            "primary".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 216,
                    s: 83.61,
                    l: 52.16,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
            ),
        );

        colors.insert(
            "secondary".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 145,
                    s: 51.0,
                    l: 66.0,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
            ),
        );

        colors.insert(
            "accent".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 60,
                    s: 4.8,
                    l: 95.9,
                },
                HslColor {
                    h: 24,
                    s: 9.8,
                    l: 10.0,
                },
            ),
        );

        colors.insert(
            "muted".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 60,
                    s: 4.8,
                    l: 95.9,
                },
                HslColor {
                    h: 25,
                    s: 5.3,
                    l: 44.7,
                },
            ),
        );

        colors.insert(
            "destructive".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 1,
                    s: 69.29,
                    l: 52.75,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
            ),
        );

        colors.insert(
            "success".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 100,
                    s: 65.0,
                    l: 60.0,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
            ),
        );

        colors.insert(
            "border".to_string(),
            ColorChoice::Simple(HslColor {
                h: 20,
                s: 2.0,
                l: 80.0,
            }),
        );

        colors.insert(
            "input".to_string(),
            ColorChoice::Simple(HslColor {
                h: 0,
                s: 0.0,
                l: 80.0,
            }),
        );

        colors.insert(
            "popover".to_string(),
            ColorChoice::Simple(HslColor {
                h: 0,
                s: 0.0,
                l: 96.08,
            }),
        );

        Self {
            name: "root".to_string(),
            colors,
            radius: RadiusCss("5px".to_string()),
        }
    }
}

impl Theme {
    fn dark() -> Self {
        let mut colors = HashMap::new();
        colors.insert(
            "background".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 214,
                    s: 15.22,
                    l: 18.04,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 90.0,
                },
            ),
        );

        colors.insert(
            "primary".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 216,
                    s: 83.61,
                    l: 52.16,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
            ),
        );

        colors.insert(
            "secondary".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 145,
                    s: 51.0,
                    l: 50.0,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
            ),
        );

        colors.insert(
            "accent".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 12,
                    s: 6.5,
                    l: 15.1,
                },
                HslColor {
                    h: 60,
                    s: 9.1,
                    l: 97.8,
                },
            ),
        );

        colors.insert(
            "muted".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 12,
                    s: 6.5,
                    l: 15.1,
                },
                HslColor {
                    h: 24,
                    s: 5.4,
                    l: 63.9,
                },
            ),
        );

        colors.insert(
            "destructive".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 1,
                    s: 69.29,
                    l: 52.75,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
            ),
        );

        colors.insert(
            "success".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 100,
                    s: 65.0,
                    l: 40.0,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
            ),
        );

        colors.insert(
            "border".to_string(),
            ColorChoice::Simple(HslColor {
                h: 240,
                s: 5.0,
                l: 50.0,
            }),
        );

        colors.insert(
            "input".to_string(),
            ColorChoice::Simple(HslColor {
                h: 240,
                s: 5.0,
                l: 50.0,
            }),
        );

        colors.insert(
            "popover".to_string(),
            ColorChoice::Simple(HslColor {
                h: 214,
                s: 15.22,
                l: 18.04,
            }),
        );

        Self {
            name: "dark".to_string(),
            colors,
            radius: RadiusCss("5px".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorChoice {
    Simple(HslColor),
    Duo(HslColor, HslColor),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HslColor {
    h: i64,
    s: f64,
    l: f64,
}

impl ToStyle for HslColor {
    fn to_style(&self) -> String {
        format!("hsl({}deg {}% {}%)", self.h, self.s, self.l)
    }
}

impl HslColor {
    pub fn try_new_from_hex(hex: &str) -> Result<Self, Box<dyn Error>> {
        let hex = hex.trim_start_matches('#');

        // Parse the hex string
        let rgb: u32 = u32::from_str_radix(hex, 16)?;

        // Extract RGB components
        let r = ((rgb >> 16) & 255) as f64 / 255.0;
        let g = ((rgb >> 8) & 255) as f64 / 255.0;
        let b = (rgb & 255) as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let diff = max - min;

        // Calculate HSL components
        let h = if diff == 0.0 {
            0.0
        } else if max == r {
            (g - b) / diff % 6.0
        } else if max == g {
            (b - r) / diff + 2.0
        } else {
            (r - g) / diff + 4.0
        };

        let h = h * 60.0;
        let l = (max + min) / 2.0;
        let s = if diff == 0.0 {
            0.0
        } else {
            diff / (1.0 - (2.0 * l - 1.0).abs())
        };

        let s_trunc = format!("{:.3}", s * 100.0);
        let s = s_trunc.parse::<f64>()?;

        let l_trunc = format!("{:.3}", l * 100.0);
        let l = l_trunc.parse::<f64>()?;

        Ok(Self { h: h as i64, s, l })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadiusCss(String);

impl RadiusCss {
    pub fn to_style(&self) -> String {
        self.0.clone()
    }
}

pub trait ToStyle {
    fn to_style(&self) -> String;
}

pub trait ExportToCss {
    fn export_to_css(&self) -> String;
}
