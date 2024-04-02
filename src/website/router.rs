use dioxus::prelude::*;

use super::pages::{components::sidenav::SideNavComp, header::Header};
use crate::website::app::HomePage;
use crate::website::pages::components::atoms::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
        #[route("/")]
        HomePage {},
        #[layout(SideNavComp)]
            #[route("/component/atom/button")]
            ButtonPage {},
            #[route("/component/atom/checkbox")]
            CheckboxPage {},
            #[route("/component/atom/formrange")]
            FormRangePage {},
            #[route("/component/atom/icon")]
            IconPage {},
            #[route("/component/atom/input")]
            InputPage {},
            #[route("/component/atom/label")]
            LabelPage {},
            #[route("/component/atom/separator")]
            SeparatorPage {},
            #[route("/component/atom/textarea")]
            TextAreaPage {},
            #[route("/component/atom/toggle")]
            TogglePage {},
        #[end_layout]
    #[route("/..route")]
    NotFound {}
}

fn NotFound() -> Element {
    rsx!( div { "404 Not Found" } )
}
