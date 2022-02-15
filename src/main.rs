use yew::prelude::*;

mod components;

use components::layout::Layout;
use components::youtube::YouTube;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct State {
    pub link: String,
    pub autoloop: bool,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state_eq(|| State {
        link: "".to_owned(),
        autoloop: true,
    });

    let set_state = {
        let state = state.clone();
        Callback::from(move |s: State| state.set(s))
    };

    let loop_onclick = {
        let state = state.clone();
        move |_| {
            state.set(State {
                link: state.link.clone(),
                autoloop: !state.autoloop,
            })
        }
    };

    html! {
        <ContextProvider<State> context={(*state).clone()}>
            <YouTube link={state.link.clone()} autoloop={state.autoloop} />
            <Layout class="absolute z-10" {set_state}>
                <div class="w-full flex items-center justify-center mt-auto">
                    <button
                        class={classes!(
                            "rounded-full p-2 border-neutral-400 border-2 mr-3".to_owned(),
                            (!state.autoloop).then(|| Some("text-neutral-400")),
                            state.autoloop.then(|| Some("text-neutral-50"))
                        )}
                    >
                        <span class="iconify w-12 h-12" data-icon="mdi:play" data-inline="false" />
                    </button>
                    <button
                        class={classes!(
                            "rounded-full p-2 border-neutral-400 border-2".to_owned(),
                            (!state.autoloop).then(|| Some("text-neutral-400")),
                            state.autoloop.then(|| Some("text-neutral-50"))
                        )}
                        onclick={loop_onclick}
                    >
                        <span class="iconify w-8 h-8" data-icon="mdi:refresh" data-inline="false" />
                    </button>
                </div>
            </Layout>
        </ContextProvider<State>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
