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
        <div class="theme-toggle">
            <button
                onclick={toggle}
                role="switch"
                aria-checked={is_dark.to_string()}
                type="button"
                class="theme-toggle"
            >
                <span class="sr-only">{ "Toggle dark mode" }</span>
                <span class="theme-toggle-track">
                    <span class="theme-toggle-thumb">
                        <svg
                            class={format!("w-3 h-3 {}", if is_dark { "text-indigo-400" } else { "text-amber-500" })}
                            fill="currentColor"
                            viewBox={if is_dark { "0 0 20 20" } else { "0 0 20 20" }}
                        >
                            {if is_dark {
                                html! { <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" /> }
                            } else {
                                html! { <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd" /> }
                            }}
                        </svg>
                    </span>
                </span>
            </button>
            <span class="text-sm text-muted">{if is_dark { "Dark" } else { "Light" }}</span>
        </div>
    }
}
