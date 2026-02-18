use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}

#[function_component(Spinner)]
pub fn spinner(props: &Props) -> Html {
    let classes = classes!(
        "inline-block",
        "align-middle",
        "text-gray-600",
        "dark:text-gray-300",
        &props.class
    );

    html! {
        <div class={classes} role="status" aria-label="Loading">
            <svg class="animate-spin h-6 w-6" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"></path>
            </svg>
        </div>
    }
}
