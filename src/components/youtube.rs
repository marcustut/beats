use js_sys::encode_uri_component;
use regex::Regex;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use yew::{classes, function_component, html, use_memo, Html, Properties};

use crate::components::loader::Loader;

#[derive(Properties, PartialEq)]
pub struct YouTubeProps {
    pub link: String,
    #[prop_or(true)]
    pub autoloop: bool,
}

#[function_component(YouTube)]
pub fn youtube(props: &YouTubeProps) -> Html {
    let YouTubeProps { link, autoloop } = props;
    let autoloop = autoloop.clone();
    let autoloop_param = use_memo(
        |&autoloop| {
            if autoloop {
                1
            } else {
                0
            }
        },
        autoloop,
    );

    let origin = encode_uri_component(
        window()
            .expect_throw("window is undefined")
            .origin()
            .as_str(),
    )
    .as_string()
    .unwrap();
    let cn = "select-none w-[110vw] h-[110vh] absolute top-1/2 left-1/2 border-none transform -translate-x-1/2 -translate-y-1/2 pointer-events-none".to_owned();

    let re = Regex::new(r#"(youtu\.be/|youtube\.com/(watch\?(.*&)?v=|(embed|v)/))([^\?&"'>]+)"#)
        .unwrap();

    html! {
        <div class={"absolute overflow-hidden top-0 left-0 w-[100vw] h-[100vh] youtube fill"}>
            if let Some(capture) = re.captures(link) {
                if let Some(_match) = capture.get(5) {
                    <iframe
                        class={cn}
                        src={format!(
                            "https://www.youtube.com/embed/{}?modestbranding=1&disablekb=1&iv_load_policy=3&playsinline=1&origin={}&autoplay=1&controls=0&loop={}&enablejsapi=1&widgetid=1&playlist={}",
                            _match.as_str(),
                            origin,
                            (*autoloop_param).clone(),
                            _match.as_str()
                        )}
                        title="YouTube video player"
                        frameborder={0}
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen={true}
                    />
                } else {
                    <div class={classes!(cn, "bg-black".to_owned())}>
                        <Loader class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2" />
                    </div>
                }
            } else {
                <div class={classes!(cn, "bg-black".to_owned())}>
                    <Loader class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2" />
                </div>
            }
        </div>
    }
}
