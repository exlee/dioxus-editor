#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::editor::Editor;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Editor {}
    })
}
