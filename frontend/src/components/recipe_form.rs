use crate::api;
use crate::components::image_manager::ImageManager;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;
use serde_json::json;
use shared_types::{Recipe, Ingredient, RecipeImage};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{SubmitEvent, HtmlSelectElement};
use yew::prelude::*;

fn parse_ingredients(lines: &[String]) -> Vec<Ingredient> {
    lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let trimmed = line.trim();
            let mut amount = 0.0;
            let mut unit = String::new();
            let mut name = String::new();
            let mut notes = None;

            let words: Vec<&str> = trimmed.split_whitespace().collect();
            if let Some(first) = words.first() {
                if let Ok(num) = first.parse::<f64>() {
                    amount = num;
                    let remaining = &words[1..];
                    if let Some(second) = remaining.first() {
                        unit = second.to_string();
                        name = remaining[1..].join(" ");
                    } else {
                        name = remaining.join(" ");
                    }
                } else {
                    name = trimmed.to_string();
                }
            }

            if let Some(start) = name.find('(') {
                if let Some(end) = name.find(')') {
                    notes = Some(name[start + 1..end].trim().to_string());
                    name = format!("{}{}", &name[..start].trim(), &name[end + 1..].trim());
                }
            }

            Ingredient {
                name: name.trim().to_string(),
                amount,
                unit,
                notes,
            }
        })
        .collect()
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_saved: Callback<()>,
    pub editing: Option<Recipe>,
    #[prop_or_default]
    pub on_refresh: Callback<()>,
}

#[function_component(RecipeForm)]
pub fn recipe_form(props: &Props) -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let title = use_state(|| props.editing.as_ref().map(|r| r.title.clone()).unwrap_or_default());
    let short = use_state(|| props.editing.as_ref().and_then(|r| r.short_description.clone()).unwrap_or_default());
    let ingredients_text = use_state(|| {
        if let Some(r) = &props.editing {
            r.ingredients.iter().map(|ing| {
                let mut parts = Vec::new();
                if ing.amount != 0.0 { parts.push(ing.amount.to_string()); }
                if !ing.unit.is_empty() { parts.push(ing.unit.clone()); }
                parts.push(ing.name.clone());
                if let Some(notes) = &ing.notes { parts.push(format!("({})", notes)); }
                parts.join(" ")
            }).collect::<Vec<String>>().join("\n")
        } else { String::new() }
    });
    let steps_text = use_state(|| {
        if let Some(r) = &props.editing {
            if let Some(arr) = r.steps.as_array() {
                return arr.iter().map(|v| v.as_str().unwrap_or(&v.to_string()).to_string()).collect::<Vec<String>>().join("\n");
            }
        }
        String::new()
    });
    let prep_minutes = use_state(|| props.editing.as_ref().and_then(|r| r.prep_minutes).unwrap_or_default());
    let cook_minutes = use_state(|| props.editing.as_ref().and_then(|r| r.cook_minutes).unwrap_or_default());
    let servings = use_state(|| props.editing.as_ref().and_then(|r| r.servings).unwrap_or_default());
    let notes = use_state(|| props.editing.as_ref().and_then(|r| r.notes.clone()).unwrap_or_default());
    let categories = use_state(|| Vec::<shared_types::Category>::new());
    let selected_category = use_state(|| props.editing.as_ref().and_then(|r| r.categories.first()).and_then(|c| c.id));
    let new_category_name = use_state(|| String::new());
    let images = use_state(|| props.editing.as_ref().map(|r| r.images.clone()).unwrap_or_default());
    let current_recipe_id = use_state(|| props.editing.as_ref().and_then(|r| r.id));

    {
        let current_recipe_id = current_recipe_id.clone();
        let editing_id = props.editing.as_ref().and_then(|r| r.id);
        use_effect_with(editing_id, move |id| {
            current_recipe_id.set(*id);
            || ()
        });
    }

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
        let current_recipe_id = current_recipe_id.clone();
        let images = images.clone();
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
            let current_recipe_id = current_recipe_id.clone();
            let images = images.clone();
            spawn_local(async move {
                let ingredients_lines: Vec<String> = ingredients_text.split('\n').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                let steps_lines: Vec<String> = steps_text.split('\n').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

                let recipe = Recipe {
                    id: editing.as_ref().and_then(|r| r.id),
                    title: (*title).clone(),
                    slug: None,
                    short_description: if (*short).is_empty() { None } else { Some((*short).clone()) },
                    ingredients: parse_ingredients(&ingredients_lines),
                    steps: json!(steps_lines),
                    prep_minutes: if *prep_minutes > 0 { Some(*prep_minutes) } else { None },
                    cook_minutes: if *cook_minutes > 0 { Some(*cook_minutes) } else { None },
                    servings: if *servings > 0 { Some(*servings) } else { None },
                    notes: if (*notes).is_empty() { None } else { Some((*notes).clone()) },
                    author_id: None,
                    is_public: Some(true),
                    categories: Vec::new(),
                    images: (*images).clone(),
                };

                let res = if let Some(id) = recipe.id {
                    api::update_recipe(id, &recipe).await.map_err(|e| e)
                } else {
                    api::create_recipe(&recipe).await.map(|r| r).map_err(|e| e)
                };

                if let Ok(created) = res {
                    if let Some(rid) = created.id {
                        current_recipe_id.set(Some(rid));
                        if let Some(cid) = *selected_category {
                            let _ = api::assign_category(rid, cid).await;
                        } else {
                            let _ = api::clear_categories(rid).await;
                        }
                    }
                }
                on_saved.emit(());
            });
        })
    };

    let on_saved_prop = props.on_saved.clone();

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
        <form onsubmit={onsubmit} class="card page-enter recipe-form">
            <div class="card-body">
                <div class="form-row">
                    <div class="form-group">
                        <input
                            placeholder={t("recipe_title", lang)}
                            value={(*title).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                title.set(input.value());
                            })}
                            class="form-input"
                        />
                    </div>
                    <div class="form-group">
                        <input
                            placeholder={t("recipe_short_desc", lang)}
                            value={(*short).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                short.set(input.value());
                            })}
                            class="form-input"
                        />
                    </div>
                </div>

                <div class="form-group">
                    <label class="form-label">{ t("ingredients_one_per_line", lang) }</label>
                    <p class="form-hint mb-2">
                        { t("ingredient_format", lang) }
                        <br />
                        { t("ingredient_examples", lang) }
                    </p>
                    <textarea
                        value={(*ingredients_text).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                            ingredients_text.set(input.value());
                        })}
                        placeholder={t("ingredient_examples_placeholder", lang)}
                        class="form-textarea"
                        rows={4}
                    />
                </div>

                <div class="form-group">
                    <label class="form-label">{ t("steps_one_per_line", lang) }</label>
                    <textarea
                        value={(*steps_text).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                            steps_text.set(input.value());
                        })}
                        class="form-textarea"
                        rows={4}
                    />
                </div>

                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">{ t("prep_time_minutes", lang) }</label>
                        <input
                            type="number"
                            value={(*prep_minutes).to_string()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                prep_minutes.set(input.value().parse::<i32>().unwrap_or(0));
                            })}
                            class="form-input"
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label">{ t("cook_time_minutes", lang) }</label>
                        <input
                            type="number"
                            value={(*cook_minutes).to_string()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                cook_minutes.set(input.value().parse::<i32>().unwrap_or(0));
                            })}
                            class="form-input"
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label">{ t("servings", lang) }</label>
                        <input
                            type="number"
                            value={(*servings).to_string()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                servings.set(input.value().parse::<i32>().unwrap_or(0));
                            })}
                            class="form-input"
                        />
                    </div>
                </div>

                <div class="form-group">
                    <label class="form-label">{ t("notes", lang) }</label>
                    <textarea
                        value={(*notes).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                            notes.set(input.value());
                        })}
                        class="form-textarea form-textarea-sm"
                        rows={3}
                    />
                </div>

                <div class="form-group">
                    <label class="form-label">{ t("category_optional", lang) }</label>
                    <select
                        onchange={Callback::from({
                            let selected_category = selected_category.clone();
                            move |e: Event| {
                                let v = e.target()
                                    .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                                    .map(|el: HtmlSelectElement| el.value())
                                    .unwrap_or_default();
                                if v.is_empty() { selected_category.set(None); }
                                else { selected_category.set(v.parse::<i32>().ok()); }
                            }
                        })}
                        class="form-select"
                    >
                        <option value="" selected={selected_category.is_none()}>{ format!("— {} —", t("none_category", lang)) }</option>
                        { for (*categories).iter().map(|c| {
                            let is_selected = c.id == *selected_category;
                            html!{ <option value={c.id.map(|id| id.to_string()).unwrap_or_default()} selected={is_selected}>{ &c.name }</option> }
                        }) }
                    </select>
                    <div class="flex gap-2 mt-2">
                        <input
                            type="text"
                            placeholder={t("new_category_name", lang)}
                            value={(*new_category_name).clone()}
                            oninput={Callback::from({
                                let new_category_name = new_category_name.clone();
                                move |e: InputEvent| {
                                    let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                    new_category_name.set(input.value());
                                }
                            })}
                            class="form-input"
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
                            class="btn btn-primary btn-sm"
                        >
                            { t("add_category", lang) }
                        </button>
                    </div>
                </div>

                <div class="form-group">
                    <ImageManager
                        recipe_id={*current_recipe_id}
                        images={(*images).clone()}
                        on_images_changed={Callback::from({
                            let images = images.clone();
                            move |new_images: Vec<RecipeImage>| { images.set(new_images); }
                        })}
                    />
                </div>

                <div class="form-actions">
                    <button type="button" onclick={Callback::from(move |_| { on_saved_prop.emit(()); })} class="btn btn-ghost">{ t("cancel", lang) }</button>
                    <button type="submit" class="btn btn-primary">{ t("save", lang) }</button>
                </div>
            </div>
        </form>
    }
}
