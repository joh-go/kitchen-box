use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::components::theme_provider::ThemeToggle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_navigate: Callback<crate::Page>,
    #[prop_or(Callback::from(|_: yew::MouseEvent| ()))]
    pub on_mobile_close: Callback<yew::MouseEvent>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &Props) -> Html {
    let on_nav = props.on_navigate.clone();
    let on_mobile_close = props.on_mobile_close.clone();
    
    // Check authentication state
    let is_logged_in = api::is_logged_in();
    let user_name = api::get_current_user_name();
    
    // Fetch recipe count
    let recipe_count = use_state(|| 0i32);
    {
        let recipe_count = recipe_count.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(recipes) = api::get_recipes().await {
                    recipe_count.set(recipes.len() as i32);
                }
            });
            || ()
        });
    }
    
    let to_home = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Home);
            on_mobile_close.emit(e);
        })
    };
    let to_add = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Add);
            on_mobile_close.emit(e);
        })
    };
    let to_login = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Login);
            on_mobile_close.emit(e);
        })
    };
    let to_register = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Register);
            on_mobile_close.emit(e);
        })
    };
    
    let to_settings = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Settings);
            on_mobile_close.emit(e);
        })
    };
    
    let on_logout = {
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            api::logout();
            on_mobile_close.emit(e);
        })
    };

    html! {
        <aside class="w-full">
            // Navigation Card
            <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-slide-in">
                // Header
                <div class="mb-6">
                    <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-2">
                        {
                            if is_logged_in {
                                if let Some(ref name) = user_name {
                                    format!("Welcome, {}", name)
                                } else {
                                    "Welcome".to_string()
                                }
                            } else {
                                "Navigation".to_string()
                            }
                        }
                    </h2>
                    <p class="text-sm text-slate-500 dark:text-slate-400">
                        {
                            if is_logged_in {
                                "Manage your recipes"
                            } else {
                                "Sign in to get started"
                            }
                        }
                    </p>
                </div>

                // Navigation Items
                <nav class="space-y-2">
                    // Home/Recipes
                    <button 
                        onclick={to_home} 
                        class="w-full touch-target flex items-center gap-3 text-left px-4 py-3 rounded-xl hover:bg-emerald-50 dark:hover:bg-slate-700 transition-all duration-200 group hover-lift"
                    >
                        <div class="w-10 h-10 bg-emerald-100 dark:bg-emerald-900/30 rounded-lg flex items-center justify-center group-hover:bg-emerald-200 dark:group-hover:bg-emerald-900/50 transition-colors">
                            <svg class="w-5 h-5 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
                            </svg>
                        </div>
                        <div class="flex-1">
                            <span class="font-medium text-slate-700 dark:text-slate-300">{"Recipes"}</span>
                            <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">{"View all recipes"}</p>
                        </div>
                        <svg class="w-4 h-4 text-slate-400 group-hover:text-emerald-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                        </svg>
                    </button>

                    // Add Recipe
                    <button 
                        onclick={to_add} 
                        class="w-full touch-target flex items-center gap-3 text-left px-4 py-3 rounded-xl hover:bg-orange-50 dark:hover:bg-slate-700 transition-all duration-200 group hover-lift"
                    >
                        <div class="w-10 h-10 bg-orange-100 dark:bg-orange-900/30 rounded-lg flex items-center justify-center group-hover:bg-orange-200 dark:group-hover:bg-orange-900/50 transition-colors">
                            <svg class="w-5 h-5 text-orange-600 dark:text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                            </svg>
                        </div>
                        <div class="flex-1">
                            <span class="font-medium text-slate-700 dark:text-slate-300">{"Add Recipe"}</span>
                            <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">{"Create new recipe"}</p>
                        </div>
                        <svg class="w-4 h-4 text-slate-400 group-hover:text-orange-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                        </svg>
                    </button>

                    // Auth Section - Show different content based on login state
                    <div class="space-y-2">
                        {if is_logged_in {
                            // Logged in user content
                            html! {
                                <>
                                    <div class="mb-4 p-3 bg-emerald-50 dark:bg-emerald-900/20 rounded-lg">
                                        <div class="flex items-center gap-2 mb-2">
                                            <div class="w-8 h-8 bg-emerald-500 rounded-full flex items-center justify-center">
                                                <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                                </svg>
                                            </div>
                                            <div class="flex-1">
                                                <span class="text-sm font-medium text-slate-700 dark:text-slate-300">{"Logged In"}</span>
                                                <p class="text-xs text-slate-500 dark:text-slate-400">{"Welcome back!"}</p>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <button 
                                        onclick={to_settings} 
                                        class="w-full touch-target flex items-center gap-3 text-left px-4 py-3 rounded-xl hover:bg-slate-50 dark:hover:bg-slate-700 transition-all duration-200 group hover-lift"
                                    >
                                        <div class="w-10 h-10 bg-slate-100 dark:bg-slate-700 rounded-lg flex items-center justify-center group-hover:bg-slate-200 dark:group-hover:bg-slate-600 transition-colors">
                                            <svg class="w-5 h-5 text-slate-600 dark:text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                            </svg>
                                        </div>
                                        <div class="flex-1">
                                            <span class="font-medium text-slate-700 dark:text-slate-300">{"Settings"}</span>
                                            <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">{"Manage your account"}</p>
                                        </div>
                                        <svg class="w-4 h-4 text-slate-400 group-hover:text-slate-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                        </svg>
                                    </button>
                                    
                                    <button 
                                        onclick={on_logout} 
                                        class="w-full touch-target flex items-center gap-3 text-left px-4 py-3 rounded-xl hover:bg-red-50 dark:hover:bg-slate-700 transition-all duration-200 group hover-lift"
                                    >
                                        <div class="w-10 h-10 bg-red-100 dark:bg-red-900/30 rounded-lg flex items-center justify-center group-hover:bg-red-200 dark:group-hover:bg-red-900/50 transition-colors">
                                            <svg class="w-5 h-5 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4 4m4-4H3"></path>
                                            </svg>
                                        </div>
                                        <div class="flex-1">
                                            <span class="font-medium text-slate-700 dark:text-slate-300">{"Logout"}</span>
                                            <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">{"Sign out of your account"}</p>
                                        </div>
                                        <svg class="w-4 h-4 text-slate-400 group-hover:text-red-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                        </svg>
                                    </button>
                                </>
                            }
                        } else {
                            // Not logged in user content
                            html! {
                                <>
                                    <button 
                                        onclick={to_login} 
                                        class="w-full touch-target flex items-center gap-3 text-left px-4 py-3 rounded-xl hover:bg-emerald-50 dark:hover:bg-slate-700 transition-all duration-200 group hover-lift"
                                    >
                                        <div class="w-10 h-10 bg-emerald-100 dark:bg-emerald-900/30 rounded-lg flex items-center justify-center group-hover:bg-emerald-200 dark:group-hover:bg-emerald-900/50 transition-colors">
                                            <svg class="w-5 h-5 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4a4 4 0 0112-4.354a6 6 0 112.354 0 018-1.18l4-4 4a4 4 0 0112-4.354 0-6.47a6 6 0 00-9.542 4.438 0 018-1.18z"></path>
                                            </svg>
                                        </div>
                                        <div class="flex-1">
                                            <span class="font-medium text-slate-700 dark:text-slate-300">{"Login"}</span>
                                            <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">{"Sign in to your account"}</p>
                                        </div>
                                    </button>

                                    <button 
                                        onclick={to_register} 
                                        class="w-full touch-target flex items-center gap-3 text-left px-4 py-3 rounded-xl hover:bg-emerald-50 dark:hover:bg-slate-700 transition-all duration-200 group hover-lift"
                                    >
                                        <div class="w-10 h-10 bg-emerald-100 dark:bg-emerald-900/30 rounded-lg flex items-center justify-center group-hover:bg-emerald-200 dark:group-hover:bg-emerald-900/50 transition-colors">
                                            <svg class="w-5 h-5 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4a4 4 0 0112-4.354a6 6 0 112.354 0 018-1.18l4-4 4a4 4 0 0112-4.354 0-6.47a6 6 0 00-9.542 4.438 0 018-1.18z"></path>
                                            </svg>
                                        </div>
                                        <div class="flex-1">
                                            <span class="font-medium text-slate-700 dark:text-slate-300">{"Register"}</span>
                                            <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">{"Create new account"}</p>
                                        </div>
                                    </button>
                                </>
                            }
                        }}
                    </div>
                </nav>

                // Divider
                <div class="my-6 border-t border-slate-200 dark:border-slate-700"></div>

                // Theme Toggle
                <div class="space-y-4">
                    <h3 class="text-sm font-medium text-slate-600 dark:text-slate-400">{"Appearance"}</h3>
                    <ThemeToggle class={Some("w-full touch-target flex items-center gap-3 text-left px-4 py-3 rounded-xl hover:bg-emerald-50 dark:hover:bg-slate-700 transition-all duration-200 group hover-lift".to_string())} />
                </div>

                // Divider
                <div class="my-6 border-t border-slate-200 dark:border-slate-700"></div>

                // Quick Stats
                <div class="space-y-4">
                    <h3 class="text-sm font-medium text-slate-600 dark:text-slate-400">{"Quick Stats"}</h3>
                    <div class="flex justify-center">
                        <div class="bg-emerald-50 dark:bg-emerald-900/20 rounded-lg p-3 text-center w-32">
                            <div class="text-2xl font-bold text-emerald-600 dark:text-emerald-400">{*recipe_count}</div>
                            <div class="text-xs text-emerald-700 dark:text-emerald-300">{"Recipes"}</div>
                        </div>
                    </div>
                </div>
            </div>
        </aside>
    }
}
