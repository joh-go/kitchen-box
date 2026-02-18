use crate::api;
use crate::models::Recipe;
use serde_json::json;
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

    let ingredients_text = use_state(|| String::new());
    let steps_text = use_state(|| String::new());
    let categories = use_state(|| Vec::<crate::models::Category>::new());
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

                let _ = api::create_recipe(&recipe).await;
                // Category assignment not yet applied server-side on create.
                let _ = &selected_category;
                on_saved.emit(());
            });
        })
    };

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
            <div>
                <input
                    placeholder="Title"
                    value={(*title).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        title.set(input.value());
                    })}
                    class="border px-2 py-1 mr-2"
                />
                <input
                    placeholder="Short description"
                    value={(*short).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        short.set(input.value());
                    })}
                    class="border px-2 py-1 mr-2"
                />
                <button type="submit" class="bg-green-500 text-white px-3 py-1">{ "Save" }</button>
            </div>

            <div class="mt-3">
                <label class="block text-sm font-medium">{ "Ingredients (one per line)" }</label>
                <textarea
                    value={(*ingredients_text).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                        ingredients_text.set(input.value());
                    })}
                    class="w-full border rounded px-2 py-1 mt-1 mb-2"
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
                    class="w-full border rounded px-2 py-1 mt-1 mb-2"
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
                    { for (*categories).iter().map(|c| html!{ <option value={c.id.to_string()}>{ &c.name }</option> }) }
                </select>
            </div>
        </form>
    }
}
