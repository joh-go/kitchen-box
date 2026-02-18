use crate::api;
use serde_json::json;
use shared_types::Recipe;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::SubmitEvent;
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
    let categories = use_state(|| Vec::<shared_types::Category>::new());
    let selected_category = use_state(|| None as Option<i32>);

    let onsubmit = {
        let title = title.clone();
        let short = short.clone();
        let ingredients_text = ingredients_text.clone();
        let steps_text = steps_text.clone();
        let selected_category = selected_category.clone();
        let on_saved = props.on_saved.clone();
        let editing = props.editing.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let title = title.clone();
            let short = short.clone();
            let ingredients_text = ingredients_text.clone();
            let steps_text = steps_text.clone();
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
                    short_description: if short.is_empty() {
                        None
                    } else {
                        Some((*short).clone())
                    },
                    ingredients: json!(ingredients_lines),
                    steps: json!(steps_lines),
                    prep_minutes: None,
                    cook_minutes: None,
                    servings: None,
                    notes: None,
                    author_id: None,
                    is_public: Some(true),
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
                        if let Some(rid) = rid {
                            let _ = api::assign_category(rid, cid).await;
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
        <form onsubmit={onsubmit} class="mb-4">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-2 items-end">
                    <div class="md:col-span-2">
                        <input
                            placeholder="Title"
                            value={(*title).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                title.set(input.value());
                            })}
                            class="w-full border rounded px-3 py-2"
                        />
                        <input
                            placeholder="Short description"
                            value={(*short).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                short.set(input.value());
                            })}
                            class="w-full border rounded px-3 py-2 mt-2"
                        />
                    </div>
                    <div class="flex gap-2">
                        <button type="submit" class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded">{ "Save" }</button>
                        <button type="button" onclick={Callback::from(move |_| { on_saved_prop.emit(()); })} class="bg-gray-200 hover:bg-gray-300 text-black px-4 py-2 rounded">{ "Cancel" }</button>
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

            <div class="mt-3">
                <label class="block text-sm font-medium">{ "Category (optional)" }</label>
                <select
                    onchange={Callback::from(move |e: Event| {
                        let v = e.target()
                            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                            .and_then(|el| el.get_attribute("value"))
                            .unwrap_or_default();

                        if v.is_empty() {
                            selected_category.set(None);
                        } else {
                            selected_category.set(v.parse::<i32>().ok());
                        }
                    })}
                    class="border rounded px-2 py-1 mt-1"
                >
                    <option value="">{ "— none —" }</option>
                    { for (*categories).iter().map(|c| html!{ <option value={c.id.map(|id| id.to_string()).unwrap_or_default()}>{ &c.name }</option> }) }
                </select>
            </div>
        </form>
    }
}
