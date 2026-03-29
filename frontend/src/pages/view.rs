use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use shared_types::Recipe;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
    pub on_edit: Callback<i32>,
    pub on_back: Callback<()>,
}

#[function_component(ViewRecipe)]
pub fn view_recipe(props: &Props) -> Html {
    let recipe = use_state(|| None as Option<Recipe>);
    let error = use_state(|| None as Option<String>);
    let id = props.id;
    let on_edit = props.on_edit.clone();
    let on_back = props.on_back.clone();

    {
        let recipe = recipe.clone();
        let error = error.clone();
        use_effect_with(id, move |&id| {
            let recipe = recipe.clone();
            let error = error.clone();
            spawn_local(async move {
                match api::get_recipe(id).await {
                    Ok(r) => recipe.set(Some(r)),
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    let handle_edit = {
        let on_edit = on_edit.clone();
        let id = id;
        Callback::from(move |_| {
            on_edit.emit(id);
        })
    };

    let handle_back = {
        let on_back = on_back.clone();
        Callback::from(move |_| {
            on_back.emit(());
        })
    };

    html! {
        <div class="space-y-6">
            { if let Some(r) = &*recipe {
                html! {
                    <>
                        // Header with back and edit buttons
                        <div class="flex items-center justify-between animate-fade-in">
                            <button
                                onclick={handle_back}
                                class="touch-target flex items-center space-x-2 text-slate-600 dark:text-slate-400 hover:text-emerald-600 dark:hover:text-emerald-400 transition-colors"
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
                                </svg>
                                <span>{"Back to Recipes"}</span>
                            </button>

                            {if r.author_id == api::get_current_user_id() {
                                html! {
                                    <button
                                        onclick={handle_edit}
                                        class="touch-target btn-primary text-white px-4 py-2 rounded-lg font-medium flex items-center gap-2 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                        </svg>
                                        {"Edit Recipe"}
                                    </button>
                                }
                            } else {
                                html! {}
                            }}
                        </div>

                        // Recipe Title and Description
                        <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
                            <h1 class="text-2xl sm:text-3xl font-bold text-slate-800 dark:text-slate-200 mb-2">
                                {&r.title}
                            </h1>
                            {if let Some(desc) = &r.short_description {
                                html! {
                                    <p class="text-slate-600 dark:text-slate-400">{desc}</p>
                                }
                            } else {
                                html! {}
                            }}

                            // Meta info
                            <div class="flex flex-wrap gap-4 mt-4 text-sm text-slate-500 dark:text-slate-400">
                                {if let Some(prep) = r.prep_minutes {
                                    html! {
                                        <div class="flex items-center gap-1">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                            </svg>
                                            <span>{format!("Prep: {} min", prep)}</span>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                                {if let Some(cook) = r.cook_minutes {
                                    html! {
                                        <div class="flex items-center gap-1">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z"></path>
                                            </svg>
                                            <span>{format!("Cook: {} min", cook)}</span>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                                {if let Some(servings) = r.servings {
                                    html! {
                                        <div class="flex items-center gap-1">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
                                            </svg>
                                            <span>{format!("Serves: {}", servings)}</span>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                                {if let Some(category) = r.categories.first() {
                                    html! {
                                        <div class="flex items-center gap-1">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                                            </svg>
                                            <span>{&category.name}</span>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                        </div>

                        // Ingredients
                        <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
                            <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4 flex items-center gap-2">
                                <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
                                </svg>
                                {"Ingredients"}
                            </h2>
                            <ul class="space-y-2">
                                {r.ingredients.iter().map(|ing| {
                                    html! {
                                        <li class="flex items-start gap-2 text-slate-700 dark:text-slate-300">
                                            <span class="w-2 h-2 bg-emerald-400 rounded-full mt-2 flex-shrink-0"></span>
                                            <span>
                                                {if ing.amount != 0.0 {
                                                    html! { <span class="font-medium">{ing.amount.to_string()}</span> }
                                                } else { html! {} }}
                                                {" "}{if !ing.unit.is_empty() {
                                                    html! { <span class="font-medium">{ing.unit.clone()}</span> }
                                                } else { html! {} }}
                                                {" "}{ing.name.clone()}
                                                {if let Some(notes) = &ing.notes {
                                                    html! { <span class="text-slate-500 dark:text-slate-400 italic">{" ("}{notes}{")"}</span> }
                                                } else { html! {} }}
                                            </span>
                                        </li>
                                    }
                                }).collect::<Html>()}
                            </ul>
                        </div>

                        // Steps
                        <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
                            <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4 flex items-center gap-2">
                                <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"></path>
                                </svg>
                                {"Instructions"}
                            </h2>
                            <div class="space-y-4">
                                {if let Some(arr) = r.steps.as_array() {
                                    arr.iter().filter_map(|s| s.as_str()).enumerate().map(|(idx, step_text)| {
                                        let step_num = idx + 1;
                                        html! {
                                            <div class="flex gap-4">
                                                <div class="flex-shrink-0 w-8 h-8 bg-emerald-100 dark:bg-emerald-900/30 rounded-full flex items-center justify-center">
                                                    <span class="text-sm font-semibold text-emerald-600 dark:text-emerald-400">{step_num}</span>
                                                </div>
                                                <p class="text-slate-700 dark:text-slate-300 pt-1">{step_text}</p>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                } else { html! {} }}
                            </div>
                        </div>

                        // Notes
                        {if let Some(notes) = &r.notes {
                            html! {
                                <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
                                    <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-2 flex items-center gap-2">
                                        <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                        </svg>
                                        {"Notes"}
                                    </h2>
                                    <p class="text-slate-600 dark:text-slate-400 whitespace-pre-line">{notes}</p>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </>
                }
            } else if let Some(e) = &*error {
                html! {
                    <div class="glass rounded-2xl p-6 shadow-lg border border-red-200 dark:border-red-800">
                        <p class="text-red-600 dark:text-red-400">{e}</p>
                        <button
                            onclick={handle_back}
                            class="mt-4 touch-target btn-primary text-white px-4 py-2 rounded-lg font-medium"
                        >
                            {"Go Back"}
                        </button>
                    </div>
                }
            } else {
                html! {
                    <div class="flex items-center justify-center py-12">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-500"></div>
                    </div>
                }
            }}
        </div>
    }
}
