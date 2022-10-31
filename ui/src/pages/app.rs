use clone_all::clone_all;
use common::frame::FrameJson;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew_agent::Bridged;
use yew_hooks::{use_effect_once, use_list, use_web_socket, UseListHandle};

use crate::components::frame::Frame;
use crate::components::frame_input::FrameInput;
use crate::util::request_frames;

#[function_component(App)]
pub fn app() -> Html {
    let frames: UseListHandle<FrameJson> = use_list(vec![]);
    // let frame: UseStateHandle<FrameModel> = use_state(|| FrameModel::default());

    let ws = use_web_socket("ws://localhost:8080/api/ws".to_string());

    {
        clone_all!(frames, ws);
        use_effect_with_deps(
            move |message| {
                if let Some(message) = &**message {
                    if let Ok(f) = serde_json::from_str(&message) {
                        frames.insert(0, f)
                    }
                }
                || ()
            },
            ws.message,
        );
    }

    {
        clone_all!(frames);
        use_effect_once(move || {
            // let frame = frame.clone();
            wasm_bindgen_futures::spawn_local(async move {
                frames.set(request_frames().await);
            });
            || ()
        })
    };

    html! {
        <>
            <Frame frames={frames.current().clone()} depth={0} />
            <FrameInput />
        </>
    }
}
