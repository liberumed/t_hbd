mod app;
mod state;
mod audio;
mod particles;
mod wishes;
mod screens;
mod activities;

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    mount_to_body(|| view! { <app::App /> });
}
