use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PlayerProps {
    pub video_url: String,
}

#[component]
pub fn Player(props: PlayerProps) -> Element {
    rsx! {
        div {
            class: "player-container",
            video {
                class: "video-element",
                src: "{props.video_url}",
                controls: true,
                autoplay: true,
            }
            div {
                class: "player-controls",
                button { "Play/Pause" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "0",
                    class: "progress-bar"
                }
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "50",
                    class: "volume-control"
                }
            }
        }
    }
}
