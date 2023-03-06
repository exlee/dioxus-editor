#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

mod app;
mod editor;
mod editor_data;
mod editor_state;
mod macros;
mod constants;
mod enums;
mod line;

fn main() {
    hot_reload_init!();

    dioxus_desktop::launch_cfg(app::App,
                               Config::default()
                               .with_custom_head(constants::HEAD.into())
                               .with_window(
                                   WindowBuilder::default()
                                       .with_title("Editor Demo")
                               )
    );
}
