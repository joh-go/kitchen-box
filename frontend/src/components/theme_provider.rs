use yew::prelude::*;
use crate::theme::{Theme, get_theme, set_theme};

#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    #[prop_or_default]
    pub class: String,
}

#[function_component(ThemeToggle)]
pub fn theme_toggle(_props: &ThemeToggleProps) -> Html {
    let theme = use_state(|| get_theme());
    let is_dark = matches!(*theme, Theme::Dark);

    let toggle = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let current_theme = (*theme).clone();
            let new_theme = current_theme.toggle();
            theme.set(new_theme.clone());
            set_theme(new_theme.as_str());
        })
    };

    html! {
        <label class="toggle">
            <input
                type="checkbox"
                checked={is_dark}
                onclick={toggle}
            />
            <span class="toggle-slider"></span>
            <span class="text-sm toggle-label">{if is_dark { "Dark" } else { "Light" }}</span>
        </label>
    }
}
