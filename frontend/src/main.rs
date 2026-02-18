use yew::prelude::*;
mod api;
mod components;
mod pages;

use components::sidebar::Sidebar;

#[derive(Clone, PartialEq)]
pub enum Page {
    Home,
    Add,
    Edit(i32),
    Users,
}

fn render_page(page: &Page, navigate: Callback<Page>) -> Html {
    match page {
        Page::Home => {
            let on_edit = {
                let navigate = navigate.clone();
                Callback::from(move |r: shared_types::Recipe| {
                    if let Some(id) = r.id {
                        navigate.emit(Page::Edit(id));
                    }
                })
            };

            html! { <crate::components::recipe_list::RecipeList on_edit={on_edit} refresh={0} search={String::new()} /> }
        }
        Page::Add => {
            html! { <crate::components::recipe_form::RecipeForm on_saved={Callback::from(move |_| navigate.emit(Page::Home))} editing={None} /> }
        }
        Page::Edit(id) => html! { <crate::pages::edit::EditRecipe id={*id} /> },
        Page::Users => html! { <crate::pages::users::UsersPage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let page = use_state(|| Page::Home);
    let navigate = {
        let page = page.clone();
        Callback::from(move |p: Page| {
            page.set(p);
        })
    };

    let current = (*page).clone();

    html! {
        <div class="min-h-screen bg-gradient-to-br from-emerald-50 via-white to-orange-50 dark:from-slate-900 dark:via-slate-800 dark:to-slate-900">
            // Modern Header with Mobile Menu
            <header class="glass sticky top-0 z-50 border-b border-emerald-100 dark:border-slate-700">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex items-center justify-between h-16 sm:h-20">
                        // Logo and Title
                        <div class="flex items-center space-x-3 animate-fade-in">
                            <div class="w-8 h-8 sm:w-10 sm:h-10 bg-gradient-to-br from-emerald-400 to-emerald-600 rounded-xl flex items-center justify-center shadow-lg">
                                <span class="text-white text-lg sm:text-xl">{"🍳"}</span>
                            </div>
                            <div>
                                <h1 class="text-xl sm:text-2xl font-bold bg-gradient-to-r from-emerald-600 to-emerald-800 dark:from-emerald-400 dark:to-emerald-300 bg-clip-text text-transparent">
                                    {"Recipe Book"}
                                </h1>
                                <p class="text-xs sm:text-sm text-slate-500 dark:text-slate-400 hidden sm:block">
                                    {"Your personal cookbook"}
                                </p>
                            </div>
                        </div>

                        // Mobile Menu Button (hidden on desktop)
                        <button class="lg:hidden touch-target p-2 rounded-lg hover:bg-emerald-100 dark:hover:bg-slate-700 transition-colors">
                            <svg class="w-6 h-6 text-slate-600 dark:text-slate-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </button>

                        // Desktop Navigation
                        <nav class="hidden lg:flex items-center space-x-4">
                            <div class="flex items-center space-x-2 text-sm text-slate-500 dark:text-slate-400">
                                <span class="w-2 h-2 bg-emerald-400 rounded-full animate-pulse-slow"></span>
                                <span>{"Ready to cook"}</span>
                            </div>
                        </nav>
                    </div>
                </div>
            </header>

            // Main Content Area
            <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6 sm:py-8">
                <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 lg:gap-8">
                    // Sidebar (hidden on mobile, shown on desktop)
                    <aside class="hidden lg:block lg:col-span-3">
                        <div class="sticky top-24">
                            <Sidebar on_navigate={navigate.clone()} />
                        </div>
                    </aside>

                    // Main Content
                    <div class="lg:col-span-9">
                        <div class="animate-fade-in">
                            { render_page(&current, navigate.clone()) }
                        </div>
                    </div>
                </div>
            </main>

            // Mobile Bottom Navigation (shown only on mobile)
            <nav class="lg:hidden fixed bottom-0 left-0 right-0 glass border-t border-emerald-100 dark:border-slate-700 z-40">
                <div class="grid grid-cols-4 gap-1">
                    <button class="touch-target flex flex-col items-center justify-center py-3 px-2 text-emerald-600 dark:text-emerald-400 hover:bg-emerald-50 dark:hover:bg-slate-800 transition-colors">
                        <svg class="w-5 h-5 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
                        </svg>
                        <span class="text-xs font-medium">{"Home"}</span>
                    </button>
                    
                    <button class="touch-target flex flex-col items-center justify-center py-3 px-2 text-slate-600 dark:text-slate-400 hover:bg-emerald-50 dark:hover:bg-slate-800 transition-colors">
                        <svg class="w-5 h-5 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                        </svg>
                        <span class="text-xs font-medium">{"Add"}</span>
                    </button>
                    
                    <button class="touch-target flex flex-col items-center justify-center py-3 px-2 text-slate-600 dark:text-slate-400 hover:bg-emerald-50 dark:hover:bg-slate-800 transition-colors">
                        <svg class="w-5 h-5 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                        </svg>
                        <span class="text-xs font-medium">{"Users"}</span>
                    </button>
                    
                    <button class="touch-target flex flex-col items-center justify-center py-3 px-2 text-slate-600 dark:text-slate-400 hover:bg-emerald-50 dark:hover:bg-slate-800 transition-colors">
                        <svg class="w-5 h-5 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                        </svg>
                        <span class="text-xs font-medium">{"Settings"}</span>
                    </button>
                </div>
            </nav>

            // Add padding for mobile bottom nav
            <div class="lg:hidden h-20"></div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
