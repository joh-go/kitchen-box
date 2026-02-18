use yew::prelude::*;

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
    let to_users = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Users);
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
                        {"Navigation"}
                    </h2>
                    <p class="text-sm text-slate-500 dark:text-slate-400">
                        {"Manage your recipes"}
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

                    // Users
                    <button 
                        onclick={to_users} 
                        class="w-full touch-target flex items-center gap-3 text-left px-4 py-3 rounded-xl hover:bg-blue-50 dark:hover:bg-slate-700 transition-all duration-200 group hover-lift"
                    >
                        <div class="w-10 h-10 bg-blue-100 dark:bg-blue-900/30 rounded-lg flex items-center justify-center group-hover:bg-blue-200 dark:group-hover:bg-blue-900/50 transition-colors">
                            <svg class="w-5 h-5 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path>
                            </svg>
                        </div>
                        <div class="flex-1">
                            <span class="font-medium text-slate-700 dark:text-slate-300">{"Users"}</span>
                            <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">{"Manage users"}</p>
                        </div>
                        <svg class="w-4 h-4 text-slate-400 group-hover:text-blue-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                        </svg>
                    </button>
                </nav>

                // Divider
                <div class="my-6 border-t border-slate-200 dark:border-slate-700"></div>

                // Quick Stats
                <div class="space-y-4">
                    <h3 class="text-sm font-medium text-slate-600 dark:text-slate-400">{"Quick Stats"}</h3>
                    <div class="grid grid-cols-2 gap-3">
                        <div class="bg-emerald-50 dark:bg-emerald-900/20 rounded-lg p-3 text-center">
                            <div class="text-2xl font-bold text-emerald-600 dark:text-emerald-400">{"12"}</div>
                            <div class="text-xs text-emerald-700 dark:text-emerald-300">{"Recipes"}</div>
                        </div>
                        <div class="bg-orange-50 dark:bg-orange-900/20 rounded-lg p-3 text-center">
                            <div class="text-2xl font-bold text-orange-600 dark:text-orange-400">{"3"}</div>
                            <div class="text-xs text-orange-700 dark:text-orange-300">{"Categories"}</div>
                        </div>
                    </div>
                </div>
            </div>
        </aside>
    }
}
