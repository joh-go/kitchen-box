use yew::prelude::*;
use crate::theme::{Theme, get_theme, set_theme};

#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    pub class: Option<String>,
}

#[function_component(ThemeToggle)]
pub fn theme_toggle(props: &ThemeToggleProps) -> Html {
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

    let class = props.class.clone().unwrap_or_else(|| "w-full touch-target flex items-center gap-3 text-left px-4 py-3 rounded-xl hover:bg-emerald-50 dark:hover:bg-slate-700 transition-all duration-200 group hover-lift".to_string());

    html! {
        <button 
            onclick={toggle}
            class={class}
        >
            <div class="w-10 h-10 bg-slate-100 dark:bg-slate-800 rounded-lg flex items-center justify-center group-hover:bg-slate-200 dark:group-hover:bg-slate-700 transition-colors">
                if is_dark {
                    <svg class="w-5 h-5 text-slate-600 dark:text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>
                    </svg>
                } else {
                    <svg class="w-5 h-5 text-slate-600 dark:text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path>
                    </svg>
                }
            </div>
            <div class="flex-1">
                <span class="font-medium text-slate-700 dark:text-slate-300">
                    {if is_dark { "Light Mode" } else { "Dark Mode" }}
                </span>
                <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
                    {if is_dark { "Switch to light theme" } else { "Switch to dark theme" }}
                </p>
            </div>
            <svg class="w-4 h-4 text-slate-400 group-hover:text-slate-600 dark:group-hover:text-slate-300 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
            </svg>
        </button>
    }
}
