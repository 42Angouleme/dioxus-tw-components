#![allow(non_snake_case)]

use dioxus::html::geometry::*;

pub mod attributes;

mod components;
pub use components::*;

pub mod bootstrap;

pub mod hooks;

pub mod prelude;

#[cfg(feature = "theme")]
pub mod theme;

pub use dioxus_tw_components_sorttable_macro::Row;

pub struct LibState {
    last_click_coordinates: Coordinates,
}

impl std::default::Default for LibState {
    fn default() -> Self {
        Self {
            last_click_coordinates: Coordinates::new(
                ScreenPoint::zero(),
                ClientPoint::zero(),
                ElementPoint::zero(),
                PagePoint::zero(),
            ),
        }
    }
}

impl LibState {
    pub fn get_last_click_coordinates(&self) -> &Coordinates {
        &self.last_click_coordinates
    }

    pub fn set_last_click_coordinates(&mut self, coordinates: Coordinates) {
        self.last_click_coordinates = coordinates;
    }
}

extern crate slugify;
