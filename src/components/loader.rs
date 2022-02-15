use yew::{classes, function_component, html, Classes, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct LoaderProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Loader)]
pub fn loader(props: &LoaderProps) -> Html {
    let LoaderProps { class } = props;

    html! {
        <span
            class={classes!("iconify text-white w-8 h-8".to_owned(), class.clone())}
            data-icon="eos-icons:loading"
            data-inline="false"
        />
    }
}
