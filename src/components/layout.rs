use yew::{
    classes, function_component, html, use_context, use_effect, Callback, Children, Classes, Html,
    Properties,
};

use crate::{
    components::sidenav::{SideNav, YouTubeVideo},
    State,
};

#[derive(PartialEq, Properties)]
pub struct LayoutProps {
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
    pub set_state: Callback<State>,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let LayoutProps {
        class,
        children,
        set_state,
    } = props;
    let state = use_context::<State>().expect("no state found");

    let beats = vec![
        YouTubeVideo {
            link: String::from("https://www.youtube.com/watch?v=aMyO6GNkfpo"),
            name: String::from("keshi - beside you"),
        },
        YouTubeVideo {
            link: String::from("https://www.youtube.com/watch?v=mtoeTzYKyaQ"),
            name: String::from("keshi - less of you"),
        },
        YouTubeVideo {
            link: String::from("https://www.youtube.com/watch?v=eAs4Z_9I5LA"),
            name: String::from("yihuik - estrangement"),
        },
        YouTubeVideo {
            link: String::from("https://www.youtube.com/watch?v=a7fzkqLozwA"),
            name: String::from("lauv - i like me better"),
        },
        YouTubeVideo {
            link: String::from("https://www.youtube.com/watch?v=6a3Pwb_c6fI&list=RDGMEMQ1dJ7wXfLlqCjwV0xfSNbA&start_radio=1&rv=a7fzkqLozwA"),
            name: String::from("priscilla abby - 夜空中最亮的星"),
        },
        YouTubeVideo {
            link: String::from("https://www.youtube.com/watch?v=3MeCh9OapjE"),
            name: String::from("My菜 - 菲道尔"),
        },
        YouTubeVideo {
            link: String::from("https://www.youtube.com/watch?v=dos9LST6kbU"),
            name: String::from("菲道尔 Firdhaus【月亮失約了Absent Moon】"),
        },
    ];

    {
        let set_state = set_state.clone();
        let beats = beats.clone();
        use_effect(move || {
            if state.link == "" {
                set_state.emit(State {
                    link: beats[0].link.clone(),
                    autoloop: state.autoloop,
                });
            }
            || {}
        });
    }

    html! {
        <div class={classes!(String::from("text-neutral-50 py-8 px-8 flex w-full h-[100vh] overflow-hidden"), class.clone())}>
            <SideNav {set_state} beats={beats} />
            { for children.iter() }
        </div>
    }
}
