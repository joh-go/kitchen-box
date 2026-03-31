use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use shared_types::{Recipe, Ingredient};

fn calculate_adjusted_ingredients(ingredients: &[Ingredient], original_servings: i32, target_servings: i32) -> Vec<(Ingredient, f64)> {
    let multiplier = target_servings as f64 / original_servings as f64;
    ingredients
        .iter()
        .map(|ing| {
            let adjusted_amount = ing.amount * multiplier;
            (ing.clone(), adjusted_amount)
        })
        .collect()
}

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
    let adjusted_servings = use_state(|| None as Option<i32>);
    let completed_steps = use_state(|| Vec::<bool>::new());
    let lightbox_open = use_state(|| false);
    let lightbox_index = use_state(|| 0usize);
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

    let on_serving_change = {
        let adjusted_servings = adjusted_servings.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(value) = input.value().parse::<i32>() {
                if value > 0 {
                    adjusted_servings.set(Some(value));
                }
            }
        })
    };

    let reset_servings = {
        let adjusted_servings = adjusted_servings.clone();
        let recipe = recipe.clone();
        Callback::from(move |_: yew::MouseEvent| {
            if let Some(r) = &*recipe {
                if let Some(original) = r.servings {
                    adjusted_servings.set(Some(original));
                }
            }
        })
    };

    let toggle_step = {
        let completed_steps = completed_steps.clone();
        Callback::from(move |step_index: usize| {
            let mut steps = (*completed_steps).clone();
            if step_index < steps.len() {
                steps[step_index] = !steps[step_index];
            } else {
                // Extend the vector if needed
                while steps.len() <= step_index {
                    steps.push(false);
                }
                steps[step_index] = true;
            }
            completed_steps.set(steps);
        })
    };

    // Lightbox navigation callbacks
    let open_lightbox = {
        let lightbox_open = lightbox_open.clone();
        let lightbox_index = lightbox_index.clone();
        Callback::from(move |index: usize| {
            lightbox_index.set(index);
            lightbox_open.set(true);
        })
    };

    let close_lightbox = {
        let lightbox_open = lightbox_open.clone();
        Callback::from(move |_: MouseEvent| {
            lightbox_open.set(false);
        })
    };

    let next_image = {
        let lightbox_index = lightbox_index.clone();
        let recipe = recipe.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(r) = &*recipe {
                let total = r.images.len();
                if total > 0 {
                    lightbox_index.set((*lightbox_index + 1) % total);
                }
            }
        })
    };

    let prev_image = {
        let lightbox_index = lightbox_index.clone();
        let recipe = recipe.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(r) = &*recipe {
                let total = r.images.len();
                if total > 0 {
                    if *lightbox_index == 0 {
                        lightbox_index.set(total - 1);
                    } else {
                        lightbox_index.set(*lightbox_index - 1);
                    }
                }
            }
        })
    };

    html! {
        <div class="space-y-6">
            { if let Some(r) = &*recipe {
                // Calculate adjusted ingredients outside the html! macro
                let current_servings = (*adjusted_servings).or(r.servings).unwrap_or(1);
                let original_servings = r.servings.unwrap_or(1);
                let adjusted_ingredients = calculate_adjusted_ingredients(&r.ingredients, original_servings, current_servings);
                
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
                                    let current_servings = (*adjusted_servings).unwrap_or(servings);
                                    if current_servings != servings {
                                        html! {
                                            <span class="text-sm text-emerald-600 dark:text-emerald-400 ml-2">
                                                {format!("(adjusted for {} servings)", current_servings)}
                                            </span>
                                        }
                                    } else {
                                        html! {}
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

                            // Primary Image
                            {if let Some((primary_idx, primary_image)) = r.images.iter().enumerate().find(|(_, img)| img.is_primary == Some(true)) {
                                let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}", 
                                    r.id.unwrap_or(0), primary_image.filename);
                                let open_lightbox = open_lightbox.clone();
                                html! {
                                    <div class="mt-6 rounded-xl overflow-hidden shadow-lg cursor-pointer hover:shadow-xl transition-shadow"
                                         onclick={Callback::from(move |_| open_lightbox.emit(primary_idx))}>
                                        <img 
                                            src={image_url}
                                            alt={primary_image.alt.clone().unwrap_or_else(|| r.title.clone())}
                                            class="w-full h-64 md:h-80 object-cover"
                                        />
                                    </div>
                                }
                            } else if let Some(first_image) = r.images.first() {
                                let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}", 
                                    r.id.unwrap_or(0), first_image.filename);
                                let open_lightbox = open_lightbox.clone();
                                html! {
                                    <div class="mt-6 rounded-xl overflow-hidden shadow-lg cursor-pointer hover:shadow-xl transition-shadow"
                                         onclick={Callback::from(move |_| open_lightbox.emit(0))}>
                                        <img 
                                            src={image_url}
                                            alt={first_image.alt.clone().unwrap_or_else(|| r.title.clone())}
                                            class="w-full h-64 md:h-80 object-cover"
                                        />
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>

                        // Ingredients
                        <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
                            <div class="flex items-center justify-between mb-4">
                                <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 flex items-center gap-2">
                                    <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
                                    </svg>
                                    {"Ingredients"}
                                    {if let Some(servings) = r.servings {
                                        let current_servings = (*adjusted_servings).unwrap_or(servings);
                                        if current_servings != servings {
                                            html! {
                                                <span class="text-sm text-emerald-600 dark:text-emerald-400 ml-2">
                                                    {format!("(adjusted for {} servings)", current_servings)}
                                                </span>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </h2>
                                {if let Some(original_servings) = r.servings {
                                    html! {
                                        <div class="flex items-center gap-2">
                                            <label class="text-sm font-medium text-slate-600 dark:text-slate-400">{"Servings:"}</label>
                                            <input
                                                type="number"
                                                min="1"
                                                value={(*adjusted_servings).unwrap_or(original_servings).to_string()}
                                                onchange={on_serving_change}
                                                class="w-16 px-2 py-1 text-sm border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
                                            />
                                            <button
                                                onclick={reset_servings}
                                                class="text-xs px-2 py-1 text-slate-500 dark:text-slate-400 hover:text-emerald-600 dark:hover:text-emerald-400 transition-colors"
                                            >
                                                {"Reset"}
                                            </button>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                            <ul class="space-y-2">
                                {adjusted_ingredients.iter().map(|(ing, adjusted_amount)| {
                                    html! {
                                        <li class="flex items-start gap-2 text-slate-700 dark:text-slate-300">
                                            <span class="w-2 h-2 bg-emerald-400 rounded-full mt-2 flex-shrink-0"></span>
                                            <span>
                                                {if *adjusted_amount != 0.0 {
                                                    html! { 
                                                        <span class="font-medium">
                                                            {format!("{:.2}", adjusted_amount).trim_end_matches(".00")}
                                                        </span> 
                                                    }
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
                                    // Ensure completed_steps vector has the right length
                                    {
                                        let mut steps = (*completed_steps).clone();
                                        while steps.len() < arr.len() {
                                            steps.push(false);
                                        }
                                        if steps.len() != arr.len() {
                                            completed_steps.set(steps);
                                        }
                                    }
                                    
                                    arr.iter().filter_map(|s| s.as_str()).enumerate().map(|(idx, step_text)| {
                                        let is_completed = if idx < (*completed_steps).len() {
                                            (*completed_steps)[idx]
                                        } else {
                                            false
                                        };
                                        
                                        html! {
                                            <div 
                                                class={if is_completed { "flex gap-4 opacity-60 cursor-pointer items-start" } else { "flex gap-4 cursor-pointer items-start" }}
                                                onclick={
                                                    let toggle_step = toggle_step.clone();
                                                    Callback::from(move |_| toggle_step.emit(idx))
                                                }
                                            >
                                                <div class="flex items-center gap-3">
                                                    <input
                                                        type="checkbox"
                                                        checked={is_completed}
                                                        onclick={Callback::from(|e: MouseEvent| {
                                                            e.stop_propagation();
                                                        })}
                                                        onchange={
                                                            let toggle_step = toggle_step.clone();
                                                            Callback::from(move |_| toggle_step.emit(idx))
                                                        }
                                                        class="w-5 h-5 text-emerald-600 bg-white dark:bg-slate-800 border-2 border-slate-300 dark:border-slate-600 rounded-md focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 cursor-pointer transition-all duration-200 hover:border-emerald-400 dark:hover:border-emerald-500 appearance-none checked:bg-emerald-600 checked:border-emerald-600"
                                                    />
                                                    <span class={if is_completed { "flex-shrink-0 w-8 h-8 bg-emerald-300 text-white rounded-full flex items-center justify-center font-semibold text-sm" } else { "flex-shrink-0 w-8 h-8 bg-emerald-500 text-white rounded-full flex items-center justify-center font-semibold text-sm" }}>
                                                        {idx + 1}
                                                    </span>
                                                </div>
                                                <p class={if is_completed { "text-slate-500 dark:text-slate-400 flex-1 line-through mt-1" } else { "text-slate-700 dark:text-slate-300 flex-1 mt-1" }}>
                                                    {step_text}
                                                </p>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! {}
                                }}
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

                        // Lightbox Modal
                        {if *lightbox_open && !r.images.is_empty() {
                            let current_image = &r.images[*lightbox_index];
                            let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}", 
                                r.id.unwrap_or(0), current_image.filename);
                            let current_num = *lightbox_index + 1;
                            let total_num = r.images.len();
                            
                            html! {
                                <div 
                                    class="fixed inset-0 z-50 bg-black/90 flex items-center justify-center"
                                    onclick={close_lightbox.clone()}
                                >
                                    // Close button
                                    <button 
                                        class="absolute top-4 right-4 text-white/80 hover:text-white p-2"
                                        onclick={Callback::from(move |e: MouseEvent| {
                                            e.stop_propagation();
                                            close_lightbox.emit(e);
                                        })}
                                    >
                                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                        </svg>
                                    </button>
                                    
                                    // Previous button
                                    {if r.images.len() > 1 {
                                        html! {
                                            <button 
                                                class="absolute left-4 top-1/2 -translate-y-1/2 text-white/80 hover:text-white p-3 bg-black/30 rounded-full hover:bg-black/50 transition-colors"
                                                onclick={Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    prev_image.emit(e);
                                                })}
                                            >
                                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                                                </svg>
                                            </button>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                    
                                    // Image container
                                    <div class="max-w-5xl max-h-[85vh] px-4" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                                        <img 
                                            src={image_url}
                                            alt={current_image.alt.clone().unwrap_or_else(|| current_image.filename.clone())}
                                            class="max-w-full max-h-[80vh] object-contain rounded-lg shadow-2xl"
                                        />
                                        <div class="text-center text-white/80 mt-4">
                                            <span class="text-sm">{format!("{} / {}", current_num, total_num)}</span>
                                            {if let Some(alt) = &current_image.alt {
                                                html! { <p class="text-lg mt-1">{alt}</p> }
                                            } else {
                                                html! {}
                                            }}
                                        </div>
                                    </div>
                                    
                                    // Next button
                                    {if r.images.len() > 1 {
                                        html! {
                                            <button 
                                                class="absolute right-4 top-1/2 -translate-y-1/2 text-white/80 hover:text-white p-3 bg-black/30 rounded-full hover:bg-black/50 transition-colors"
                                                onclick={Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    next_image.emit(e);
                                                })}
                                            >
                                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                                </svg>
                                            </button>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                        {if r.images.len() > 1 {
                            html! {
                                <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
                                    <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4 flex items-center gap-2">
                                        <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                                        </svg>
                                        {"Gallery"}
                                    </h2>
                                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                                        { for r.images.iter().enumerate().map(|(idx, image)| {
                                            let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}", 
                                                r.id.unwrap_or(0), image.filename);
                                            let is_primary = image.is_primary.unwrap_or(false);
                                            let open_lightbox = open_lightbox.clone();
                                            
                                            html! {
                                                <div class="relative group cursor-pointer"
                                                     onclick={Callback::from(move |_| open_lightbox.emit(idx))}>
                                                    <div class="aspect-square bg-slate-100 dark:bg-slate-800 rounded-lg overflow-hidden hover:opacity-90 transition-opacity">
                                                        <img 
                                                            src={image_url}
                                                            alt={image.alt.clone().unwrap_or_else(|| image.filename.clone())}
                                                            class="w-full h-full object-cover"
                                                        />
                                                    </div>
                                                    { if is_primary {
                                                        html! {
                                                            <div class="absolute top-2 left-2 bg-emerald-600 text-white text-xs px-2 py-1 rounded-full">
                                                                { "Primary" }
                                                            </div>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </div>
                                            }
                                        }) }
                                    </div>
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
