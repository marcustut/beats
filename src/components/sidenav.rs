use yew::{classes, function_component, html, use_context, Callback, Classes, Html, Properties};

use crate::State;

#[derive(Debug, Clone, PartialEq)]
pub struct YouTubeVideo {
    pub link: String,
    pub name: String,
}

#[derive(Properties, PartialEq)]
pub struct SideNavProps {
    pub beats: Vec<YouTubeVideo>,
    pub set_state: Callback<State>,
}

#[function_component(SideNav)]
pub fn sidenav(props: &SideNavProps) -> Html {
    let SideNavProps { beats, set_state } = props;
    html! {
        <div class="flex flex-col h-full">
            <h1 class="uppercase tracking-widest text-4xl mb-8">{"Beats"}</h1>
            <div class="border-r-2 border-neutral-400 pr-4 overflow-hidden whitespace-nowrap">
                {for beats.iter().enumerate().map(|(i, beat)| {
                    html! {
                    <SideNavItem
                        link={beat.link.clone()}
                        name={beat.name.clone()}
                        {set_state}
                        class={classes!((i != 0).then(|| Some("mt-2")))}
                    />
                    }
                })}
            </div>
            <div class="mt-auto">
            {"Credits"}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SideNavItemProps {
    pub link: String,
    pub name: String,
    pub set_state: Callback<State>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SideNavItem)]
pub fn sidenav_item(props: &SideNavItemProps) -> Html {
    let SideNavItemProps {
        link,
        name,
        set_state,
        class,
    } = props;
    let state = use_context::<State>().expect("no state found");
    let link = link.clone();

    let onclick = {
        let set_state = set_state.clone();
        move |_| {
            set_state.emit(State {
                link: link.clone(),
                autoloop: state.autoloop,
            })
        }
    };

    html! {
        <button
            class={classes!(
                String::from("flex items-center text-left text-neutral-400 transform hover:scale-105 hover:text-neutral-50 transition duration-200 ease-in-out"),
                class.clone(),
            )}
            {onclick}
        >
            <span class="iconify mr-4" data-icon="icomoon-free:music"></span>
            { name }
        </button>
    }
}
