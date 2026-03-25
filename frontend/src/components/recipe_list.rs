use crate::api;
use shared_types::Recipe;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_edit: Callback<Recipe>,
    pub refresh: i32,
    pub search: String,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &Props) -> Html {
    let recipes = use_state(|| Vec::<Recipe>::new());
    let error = use_state(|| None::<String>);
    let is_logged_in = api::is_logged_in();
    let current_user_id = api::get_current_user_id();
    // use search passed from parent for centralized header search
    let search = props.search.clone();

    // accept refresh as external prop if passed in (backwards compatible)
    // We'll try to read a field named `refresh` via `js_sys` props are typed, so update Props accordingly in parent.

    {
        let recipes = recipes.clone();
        let error = error.clone();
        let refresh_dep = props.refresh;
        use_effect_with(refresh_dep, move |_refresh| {
            let recipes = recipes.clone();
            let error = error.clone();
            spawn_local(async move {
                match api::get_recipes().await {
                    Ok(list) => recipes.set(list),
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    let on_delete = {
        let recipes = recipes.clone();
        Callback::from(move |id: i32| {
            let recipes = recipes.clone();
            spawn_local(async move {
                if api::delete_recipe(id).await.is_ok() {
                    recipes.set(
                        recipes
                            .iter()
                            .cloned()
                            .filter(|r| r.id != Some(id))
                            .collect(),
                    );
                }
            });
        })
    };

    html! {
        <div class="space-y-6">
            // Header Section
            <div class="animate-fade-in">
                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                    <div>
                        <h1 class="text-2xl sm:text-3xl font-bold text-slate-800 dark:text-slate-200">
                            {"Your Recipes"}
                        </h1>
                        <p class="text-slate-500 dark:text-slate-400 mt-1">
                            { format!("{} delicious recipes", (*recipes).len()) }
                        </p>
                    </div>
                    
                    // Mobile Search Input
                    <div class="block lg:hidden">
                        <div class="relative">
                            <svg class="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                            </svg>
                            <input
                                type="text"
                                placeholder="Search recipes..."
                                value={search.clone()}
                                readonly=true
                                class="w-full pl-10 pr-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                            />
                        </div>
                    </div>
                </div>
            </div>

            // Error State
            {
                if let Some(e) = &*error {
                    html!{
                        <div class="glass rounded-xl p-6 border border-red-200 dark:border-red-800 animate-fade-in">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 bg-red-100 dark:bg-red-900/30 rounded-lg flex items-center justify-center">
                                    <svg class="w-5 h-5 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="font-medium text-red-800 dark:text-red-200">{"Error loading recipes"}</h3>
                                    <p class="text-sm text-red-600 dark:text-red-400 mt-1">{ e }</p>
                                </div>
                            </div>
                        </div>
                    }
                } else { html!{} }
            }

            // Recipe Grid
            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
                { for (*recipes).iter().filter(|r| {
                    if search.is_empty() { true }
                    else {
                        let q = search.to_lowercase();
                        r.title.to_lowercase().contains(&q) || r.short_description.clone().unwrap_or_default().to_lowercase().contains(&q)
                    }
                }).map(|r| {
                    let id = r.id.unwrap_or_default();
                    let r_clone = r.clone();
                    
                    // Check if recipe is owned by current user
                    let is_owned = if is_logged_in {
                        r.author_id.is_some() && 
                        current_user_id.map(|uid| uid == r.author_id.unwrap_or_default()).unwrap_or(false)
                    } else {
                        false
                    };
                    
                    html!{
                        <div class={format!("glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 card-hover animate-fade-in{}", if is_owned { " ring-2 ring-emerald-500 ring-offset-2 dark:ring-offset-slate-900" } else { "" })}>
                            // Recipe Header
                            <div class="flex items-start justify-between mb-4">
                                <div class="flex-1">
                                    <h3 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-2 line-clamp-2">
                                        { &r.title }
                                    </h3>
                                    <p class="text-sm text-slate-600 dark:text-slate-400 line-clamp-3">
                                        { r.short_description.clone().unwrap_or_default() }
                                    </p>
                                </div>
                                
                                // Owner Badge
                                {
                                    if is_owned {
                                        html! {
                                            <div class="flex items-center gap-2">
                                                <div class="px-3 py-1 bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-300 rounded-full text-xs font-medium flex items-center gap-1">
                                                    <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                                    </svg>
                                                    <span class="font-semibold">{"Owner"}</span>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                
                                // Prep Time Badge
                                if let Some(prep_time) = r.prep_minutes {
                                    <div class="ml-4 flex items-center gap-1 bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-300 px-3 py-1 rounded-full text-sm font-medium">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                        { format!("{}m", prep_time) }
                                    </div>
                                }
                            </div>

                            // Recipe Categories (if available)
                            // Note: Categories not available in Recipe struct yet
                            // { if !r.categories.is_empty() {
                            //     html!{
                            //         <div class="flex flex-wrap gap-2 mb-4">
                            //             { for r.categories.iter().take(3).map(|cat| {
                            //                 html!{
                            //                     <span class="px-2 py-1 bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300 rounded-md text-xs font-medium">
                            //                         { &cat.name }
                            //                     </span>
                            //                 }
                            //             })}
                            //             { if r.categories.len() > 3 {
                            //                 html!{
                            //                     <span class="px-2 py-1 bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400 rounded-md text-xs font-medium">
                            //                         { format!("+{}", r.categories.len() - 3) }
                            //                     </span>
                            //                 }
                            //             }}
                            //         </div>
                            //     }
                            // } else { html!{} } }

                            // Action Buttons - Only show for owned recipes
                            {if is_owned {
                                html! {
                                    <div class="flex gap-3">
                                        <button 
                                            class="flex-1 touch-target btn-primary text-white px-4 py-2.5 rounded-lg font-medium flex items-center justify-center gap-2 transition-all duration-200"
                                            onclick={props.on_edit.reform(move |_| r_clone.clone())}
                                        >
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                            </svg>
                                            {"Edit"}
                                        </button>
                                        <button 
                                            class="touch-target bg-red-50 hover:bg-red-100 dark:bg-red-900/20 dark:hover:bg-red-900/30 text-red-600 dark:text-red-400 px-4 py-2.5 rounded-lg font-medium flex items-center justify-center gap-2 transition-all duration-200 hover-lift"
                                            onclick={on_delete.reform(move |_| id)}
                                        >
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                                            </svg>
                                            {"Delete"}
                                        </button>
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                    }
                }) }
            </div>

            // Empty State
            { if (*recipes).is_empty() && error.is_none() {
                html!{
                    <div class="text-center py-12 animate-fade-in">
                        <div class="w-20 h-20 bg-slate-100 dark:bg-slate-800 rounded-full flex items-center justify-center mx-auto mb-4">
                            <svg class="w-10 h-10 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                            </svg>
                        </div>
                        <h3 class="text-lg font-medium text-slate-800 dark:text-slate-200 mb-2">
                            {"No recipes yet"}
                        </h3>
                        <p class="text-slate-500 dark:text-slate-400 mb-6">
                            {"Start building your recipe collection by adding your first recipe."}
                        </p>
                        <button class="btn-primary text-white px-6 py-3 rounded-lg font-medium inline-flex items-center gap-2">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                            </svg>
                            {"Add Your First Recipe"}
                        </button>
                    </div>
                }
            } else { html!{} } }
        </div>
    }
}
