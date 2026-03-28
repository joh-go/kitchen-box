use crate::api;
use serde_json::json;
use shared_types::Recipe;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{SubmitEvent, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_saved: Callback<()>,
    pub editing: Option<Recipe>,
}

#[function_component(RecipeForm)]
pub fn recipe_form(props: &Props) -> Html {
    let title = use_state(|| {
        props
            .editing
            .as_ref()
            .map(|r| r.title.clone())
            .unwrap_or_default()
    });
    let short = use_state(|| {
        props
            .editing
            .as_ref()
            .and_then(|r| r.short_description.clone())
            .unwrap_or_default()
    });
    let ingredients_text = use_state(|| {
        if let Some(r) = &props.editing {
            if let Some(arr) = r.ingredients.as_array() {
                return arr
                    .iter()
                    .map(|v| v.as_str().unwrap_or(&v.to_string()).to_string())
                    .collect::<Vec<String>>()
                    .join("\n");
            }
        }
        String::new()
    });

    let steps_text = use_state(|| {
        if let Some(r) = &props.editing {
            if let Some(arr) = r.steps.as_array() {
                return arr
                    .iter()
                    .map(|v| v.as_str().unwrap_or(&v.to_string()).to_string())
                    .collect::<Vec<String>>()
                    .join("\n");
            }
        }
        String::new()
    });
    let prep_minutes = use_state(|| {
        props
            .editing
            .as_ref()
            .and_then(|r| r.prep_minutes)
            .unwrap_or_default()
    });
    let cook_minutes = use_state(|| {
        props
            .editing
            .as_ref()
            .and_then(|r| r.cook_minutes)
            .unwrap_or_default()
    });
    let servings = use_state(|| {
        props
            .editing
            .as_ref()
            .and_then(|r| r.servings)
            .unwrap_or_default()
    });
    let notes = use_state(|| {
        props
            .editing
            .as_ref()
            .and_then(|r| r.notes.clone())
            .unwrap_or_default()
    });
    let categories = use_state(|| Vec::<shared_types::Category>::new());
    let selected_category = use_state(|| {
        props.editing
            .as_ref()
            .and_then(|r| r.categories.first())
            .and_then(|c| c.id)
    });
    let new_category_name = use_state(|| String::new());

    let onsubmit = {
        let title = title.clone();
        let short = short.clone();
        let ingredients_text = ingredients_text.clone();
        let steps_text = steps_text.clone();
        let prep_minutes = prep_minutes.clone();
        let cook_minutes = cook_minutes.clone();
        let servings = servings.clone();
        let notes = notes.clone();
        let selected_category = selected_category.clone();
        let on_saved = props.on_saved.clone();
        let editing = props.editing.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let title = title.clone();
            let short = short.clone();
            let ingredients_text = ingredients_text.clone();
            let steps_text = steps_text.clone();
            let prep_minutes = prep_minutes.clone();
            let cook_minutes = cook_minutes.clone();
            let servings = servings.clone();
            let notes = notes.clone();
            let selected_category = selected_category.clone();
            let on_saved = on_saved.clone();
            let editing = editing.clone();
            spawn_local(async move {
                let ingredients_lines: Vec<String> = ingredients_text
                    .split('\n')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                let steps_lines: Vec<String> = steps_text
                    .split('\n')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                let recipe = Recipe {
                    id: editing.as_ref().and_then(|r| r.id),
                    title: (*title).clone(),
                    slug: None,
                    short_description: if (*short).is_empty() {
                        None
                    } else {
                        Some((*short).clone())
                    },
                    ingredients: json!(ingredients_lines),
                    steps: json!(steps_lines),
                    prep_minutes: if *prep_minutes > 0 { Some(*prep_minutes) } else { None },
                    cook_minutes: if *cook_minutes > 0 { Some(*cook_minutes) } else { None },
                    servings: if *servings > 0 { Some(*servings) } else { None },
                    notes: if (*notes).is_empty() { None } else { Some((*notes).clone()) },
                    author_id: None,
                    is_public: Some(true),
                    categories: Vec::new(),
                };

                // Create or update
                let res = if let Some(id) = recipe.id {
                    api::update_recipe(id, &recipe).await.map_err(|e| e)
                } else {
                    api::create_recipe(&recipe).await.map(|r| r).map_err(|e| e)
                };

                if let Ok(created) = res {
                    // use returned recipe (with id)
                    let rid = created.id;
                    if let Some(cid) = *selected_category {
                        // Assign the new category
                        if let Some(rid) = rid {
                            let _ = api::assign_category(rid, cid).await;
                        }
                    } else {
                        // Clear all categories if none selected
                        if let Some(rid) = rid {
                            let _ = api::clear_categories(rid).await;
                        }
                    }
                }

                on_saved.emit(());
            });
        })
    };

    let on_saved_prop = props.on_saved.clone();

    // load categories once
    {
        let categories = categories.clone();
        let loaded = use_state(|| false);
        use_effect(move || {
            let categories = categories.clone();
            let loaded = loaded.clone();
            if !*loaded {
                spawn_local(async move {
                    if let Ok(list) = crate::api::get_categories().await {
                        categories.set(list);
                    }
                    loaded.set(true);
                });
            }
            || ()
        });
    }

    html! {
        <form onsubmit={onsubmit} class="mb-4 bg-white dark:bg-gray-800 p-4 rounded shadow">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-2 items-end">
                    <div class="md:col-span-2">
                        <input
                            placeholder="Title"
                            value={(*title).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                title.set(input.value());
                            })}
                            class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-green-300 dark:focus:ring-green-700"
                        />
                        <input
                            placeholder="Short description"
                            value={(*short).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                short.set(input.value());
                            })}
                            class="w-full border rounded px-3 py-2 mt-2 focus:outline-none focus:ring-2 focus:ring-green-200 dark:focus:ring-green-700"
                        />
                    </div>
                </div>

            <div class="mt-3">
                <label class="block text-sm font-medium">{ "Ingredients (one per line)" }</label>
                <textarea
                    value={(*ingredients_text).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                        ingredients_text.set(input.value());
                    })}
                    class="w-full border rounded px-3 py-2 mt-1 mb-2"
                    rows={4}
                />
            </div>

            <div class="mt-3">
                <label class="block text-sm font-medium">{ "Steps (one per line)" }</label>
                <textarea
                    value={(*steps_text).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                        steps_text.set(input.value());
                    })}
                    class="w-full border rounded px-3 py-2 mt-1 mb-2"
                    rows={4}
                />
            </div>

            <div class="mt-3 grid grid-cols-1 md:grid-cols-3 gap-3">
                <div>
                    <label class="block text-sm font-medium">{ "Prep Time (minutes)" }</label>
                    <input
                        type="number"
                        value={(*prep_minutes).to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                            prep_minutes.set(input.value().parse::<i32>().unwrap_or(0));
                        })}
                        class="w-full border rounded px-3 py-2 mt-1"
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium">{ "Cook Time (minutes)" }</label>
                    <input
                        type="number"
                        value={(*cook_minutes).to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                            cook_minutes.set(input.value().parse::<i32>().unwrap_or(0));
                        })}
                        class="w-full border rounded px-3 py-2 mt-1"
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium">{ "Servings" }</label>
                    <input
                        type="number"
                        value={(*servings).to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                            servings.set(input.value().parse::<i32>().unwrap_or(0));
                        })}
                        class="w-full border rounded px-3 py-2 mt-1"
                    />
                </div>
            </div>

            <div class="mt-3">
                <label class="block text-sm font-medium">{ "Notes" }</label>
                <textarea
                    value={(*notes).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                        notes.set(input.value());
                    })}
                    class="w-full border rounded px-3 py-2 mt-1 mb-2"
                    rows={3}
                />
            </div>

            <div class="mt-3">
                <label class="block text-sm font-medium">{ "Category (optional)" }</label>
                <div class="flex gap-2 mt-1">
                    <select
                        onchange={Callback::from({
                            let selected_category = selected_category.clone();
                            move |e: Event| {
                                let v = e.target()
                                    .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                                    .map(|el: HtmlSelectElement| el.value())
                                    .unwrap_or_default();

                                if v.is_empty() {
                                    selected_category.set(None);
                                } else {
                                    selected_category.set(v.parse::<i32>().ok());
                                }
                            }
                        })}
                        class="border rounded px-2 py-1 flex-1"
                    >
                        <option value="" selected={selected_category.is_none()}>{ "— none —" }</option>
                        { for (*categories).iter().map(|c| {
                            let is_selected = c.id == *selected_category;
                            html!{ 
                                <option value={c.id.map(|id| id.to_string()).unwrap_or_default()} selected={is_selected}>{ &c.name }</option> 
                            }
                        }) }
                    </select>
                </div>
                <div class="flex gap-2 mt-2">
                    <input
                        type="text"
                        placeholder="New category name"
                        value={(*new_category_name).clone()}
                        oninput={Callback::from({
                            let new_category_name = new_category_name.clone();
                            move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                new_category_name.set(input.value());
                            }
                        })}
                        class="border rounded px-2 py-1 flex-1 text-sm"
                    />
                    <button
                        type="button"
                        onclick={Callback::from(move |_| {
                            let name = (*new_category_name).clone();
                            if !name.trim().is_empty() {
                                let categories = categories.clone();
                                let new_category_name_clone = new_category_name.clone();
                                let selected_category_clone = selected_category.clone();
                                spawn_local(async move {
                                    if let Ok(created) = api::create_category(&name).await {
                                        if let Some(id) = created.get("id").and_then(|v| v.as_i64()) {
                                            selected_category_clone.set(Some(id as i32));
                                        }
                                        new_category_name_clone.set(String::new());
                                        if let Ok(list) = api::get_categories().await {
                                            categories.set(list);
                                        }
                                    }
                                });
                            }
                        })}
                        class="bg-blue-500 hover:bg-blue-600 text-white px-3 py-1 rounded text-sm"
                    >
                        { "+ Add" }
                    </button>
                </div>
            </div>
            
            <div class="flex gap-2 mt-6">
                <button type="submit" class="bg-gradient-to-r from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 text-white px-4 py-2 rounded shadow">{ "Save" }</button>
                <button type="button" onclick={Callback::from(move |_| { on_saved_prop.emit(()); })} class="bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 text-black px-4 py-2 rounded">{ "Cancel" }</button>
            </div>
        </form>
    }
}
