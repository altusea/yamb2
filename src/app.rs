use dioxus::prelude::*;
use crate::emby::client::EmbyClient;
use crate::components::Player;

#[component]
pub fn App() -> Element {
    let emby_client = use_signal(|| 
        EmbyClient::new(
            "http://your-emby-server".to_string(),
            "your-api-key".to_string()
        )
    );

    let video_url = use_signal(|| String::new());

    rsx! {
        div {
            h1 { "Emby Player" }
            Player {
                video_url: video_url()
            }
        }
    }
}