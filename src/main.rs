use dioxus::prelude::*;

mod app;
mod components;
mod emby;

fn main() {
    dioxus::launch(app::App);
}
