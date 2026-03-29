use yew::prelude::*;
use crate::theme::{Theme, get_theme, set_theme};

#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    #[prop_or_default]
    pub class: String,
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

    let default_class = "relative inline-flex items-center h-5 rounded-full w-9 transition-all duration-300 ease-in-out cursor-pointer focus:outline-none focus:ring-2 focus:ring-emerald-500/50 focus:ring-offset-2 dark:focus:ring-offset-slate-900 flex-shrink-0";
    let class = if props.class.is_empty() {
        default_class.to_string()
    } else {
        props.class.clone()
    };

    html! {
        <div class="flex items-center space-x-3">
            <button 
                onclick={toggle}
                class={class}
                role="switch"
                aria-checked={is_dark.to_string()}
                type="button"
            >
                <span class="sr-only">{ "Toggle dark mode" }</span>
                
                // Background track
                <span 
                    class={format!(
                        "absolute inset-0 rounded-full transition-all duration-300 {}",
                        if is_dark { 
                            "bg-gradient-to-r from-emerald-600 to-emerald-700 shadow-lg shadow-emerald-600/25" 
                        } else { 
                            "bg-gradient-to-r from-emerald-400 to-emerald-500 shadow-lg shadow-emerald-400/25" 
                        }
                    )}
                />
                
                // Toggle thumb
                <span 
                    class={format!(
                        "relative inline-block h-4 w-4 transform rounded-full transition-all duration-300 ease-in-out {}",
                        if is_dark { "translate-x-4" } else { "translate-x-0.5" }
                    )}
                >
                    // Thumb background
                    <span class="absolute inset-0 rounded-full bg-white shadow-md" />
                    
                    // Icons container
                    <span class="absolute inset-0 flex items-center justify-center">
                        // Sun icon (visible in light mode)
                        <svg 
                            class={format!(
                                "w-2.5 h-2.5 transition-all duration-300 {}",
                                if is_dark { "opacity-0 scale-0" } else { "opacity-100 scale-100 text-emerald-600" }
                            )}
                            fill="currentColor" 
                            viewBox="0 0 20 20"
                        >
                            <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd" />
                        </svg>
                        
                        // Moon icon (visible in dark mode)
                        <svg 
                            class={format!(
                                "w-2.5 h-2.5 transition-all duration-300 {}",
                                if is_dark { "opacity-100 scale-100 text-emerald-400" } else { "opacity-0 scale-0" }
                            )}
                            fill="currentColor" 
                            viewBox="0 0 20 20"
                        >
                            <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
                        </svg>
                    </span>
                </span>
            </button>
            
            // Mode text
            <span class="text-sm font-medium text-slate-600 dark:text-slate-300">
                {if is_dark { "Dark" } else { "Light" }}
            </span>
        </div>
    }
}
