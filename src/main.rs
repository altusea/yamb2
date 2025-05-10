use dioxus::prelude::*;

mod components;
mod emby;
mod app;

fn main() {
    dioxus::launch(app::App);
}
