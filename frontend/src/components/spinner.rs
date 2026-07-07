use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}

#[function_component(Spinner)]
pub fn spinner(props: &Props) -> Html {
    let classes = classes!(
        "spinner",
        &props.class
    );

    html! {
        <div class={classes} role="status" aria-label="Loading">
            <div class="spinner-circle"></div>
        </div>
    }
}
