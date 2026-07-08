use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;
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
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

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

    let _on_serving_change = {
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

    let _reset_servings = {
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
                while steps.len() <= step_index {
                    steps.push(false);
                }
                steps[step_index] = true;
            }
            completed_steps.set(steps);
        })
    };

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
        <div class="recipe-detail">
            { if let Some(r) = &*recipe {
                let current_servings = (*adjusted_servings).or(r.servings).unwrap_or(1);
                let original_servings = r.servings.unwrap_or(1);
                let adjusted_ingredients = calculate_adjusted_ingredients(&r.ingredients, original_servings, current_servings);

                html! {
                    <>
                        <div class="recipe-detail-header">
                            <button
                                onclick={handle_back}
                                class="recipe-detail-back"
                            >
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
                                </svg>
                                <span>{t("back_to_recipes", lang)}</span>
                            </button>
                        </div>

                        <div class="card page-enter">
                            <div class="card-body">
                                <h1 class="recipe-detail-title">{&r.title}</h1>
                                {if let Some(desc) = &r.short_description {
                                    html! { <p class="recipe-detail-desc">{desc}</p> }
                                } else { html!{} }}

                                <div class="recipe-detail-meta">
                                    {if let Some(prep) = r.prep_minutes {
                                        html! {
                                            <div class="recipe-detail-meta-item">
                                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                </svg>
                                                <div>
                                                    <div class="recipe-detail-meta-label">{t("prep_time", lang)}</div>
                                                    <div class="recipe-detail-meta-value">{format!("{} min", prep)}</div>
                                                </div>
                                            </div>
                                        }
                                    } else { html!{} }}
                                    {if let Some(cook) = r.cook_minutes {
                                        html! {
                                            <div class="recipe-detail-meta-item">
                                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z"></path>
                                                </svg>
                                                <div>
                                                    <div class="recipe-detail-meta-label">{t("cook_time", lang)}</div>
                                                    <div class="recipe-detail-meta-value">{format!("{} min", cook)}</div>
                                                </div>
                                            </div>
                                        }
                                    } else { html!{} }}
                                    {if let Some(servings) = r.servings {
                                        html! {
                                            <div class="recipe-detail-meta-item">
                                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                                </svg>
                                                <div>
                                                    <div class="recipe-detail-meta-label">{t("servings", lang)}</div>
                                                    <div class="recipe-detail-meta-value">{servings}</div>
                                                </div>
                                            </div>
                                        }
                                    } else { html!{} }}
                                    {if let Some(category) = r.categories.first() {
                                        html! {
                                            <div class="recipe-detail-meta-item">
                                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                                                </svg>
                                                <div>
                                                    <div class="recipe-detail-meta-label">{t("category", lang)}</div>
                                                    <div class="recipe-detail-meta-value">{&category.name}</div>
                                                </div>
                                            </div>
                                        }
                                    } else { html!{} }}
                                </div>

                                {if let Some((primary_idx, primary_image)) = r.images.iter().enumerate().find(|(_, img)| img.is_primary == Some(true)) {
                                    let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}",
                                        r.id.unwrap_or(0), primary_image.filename);
                                    let open = open_lightbox.clone();
                                    html! {
                                        <div class="gallery-main" onclick={Callback::from(move |_| open.emit(primary_idx))}>
                                            <img src={image_url} alt={primary_image.alt.clone().unwrap_or_else(|| r.title.clone())} />
                                        </div>
                                    }
                                } else if let Some(first_image) = r.images.first() {
                                    let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}",
                                        r.id.unwrap_or(0), first_image.filename);
                                    let open = open_lightbox.clone();
                                    html! {
                                        <div class="gallery-main" onclick={Callback::from(move |_| open.emit(0))}>
                                            <img src={image_url} alt={first_image.alt.clone().unwrap_or_else(|| r.title.clone())} />
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <div class="gallery-main">
                                            <div class="gallery-main-placeholder"></div>
                                        </div>
                                    }
                                }}
                            </div>

                            <div class="card-body">
                                {if r.author_id == api::get_current_user_id() {
                                    html! {
                                        <div class="flex justify-end mb-4">
                                            <button onclick={handle_edit} class="btn btn-primary">
                                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" style="width: 1rem; height: 1rem;">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                                </svg>
                                                {t("edit_recipe", lang)}
                                            </button>
                                        </div>
                                    }
                                } else { html!{} }}
                            </div>
                        </div>

                        <div class="ingredients-section">
                            <div class="card page-enter">
                                <div class="card-body">
                                    <div class="ingredients-header">
                                        <h2 class="section-title flex items-center gap-2">
                                            <svg width="1.25rem" height="1.25rem" fill="none" stroke="var(--primary)" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
                                            </svg>
                                            {t("ingredients", lang)}
                                        </h2>
                                        {if let Some(original_servings) = r.servings {
                                            html! {
                                                <div class="servings-control">
                                                    <label class="text-sm text-muted">{t("servings_label", lang)}</label>
                                                    <button class="servings-btn"
                                                        onclick={Callback::from({
                                                            let adjusted_servings = adjusted_servings.clone();
                                                            let r = r.clone();
                                                            move |_: yew::MouseEvent| {
                                                                if let Some(original) = r.servings {
                                                                    let current = *adjusted_servings;
                                                                    let new_val = std::cmp::max(1, current.unwrap_or(original) - 1);
                                                                    adjusted_servings.set(Some(new_val));
                                                                }
                                                            }
                                                        })}
                                                        disabled={(*adjusted_servings).unwrap_or(original_servings) <= 1}
                                                    >
                                                        {"-"}
                                                    </button>
                                                    <span class="servings-value">{current_servings}</span>
                                                    <button class="servings-btn"
                                                        onclick={Callback::from({
                                                            let adjusted_servings = adjusted_servings.clone();
                                                            let r = r.clone();
                                                            move |_: yew::MouseEvent| {
                                                                if let Some(original) = r.servings {
                                                                    let current = *adjusted_servings;
                                                                    adjusted_servings.set(Some(current.unwrap_or(original) + 1));
                                                                }
                                                            }
                                                        })}
                                                    >
                                                        {"+"}
                                                    </button>
                                                </div>
                                            }
                                        } else { html!{} }}
                                    </div>
                                    <ul class="ingredient-list">
                                        {adjusted_ingredients.iter().map(|(ing, adjusted_amount)| {
                                            html! {
                                                <li class="ingredient-item">
                                                    {if *adjusted_amount != 0.0 {
                                                        html! { <span class="ingredient-amount">{format!("{:.2}", adjusted_amount).trim_end_matches(".00").to_string()}</span> }
                                                    } else { html!{} }}
                                                    {if !ing.unit.is_empty() {
                                                        html! { <span class="ingredient-unit">{ing.unit.clone()}</span> }
                                                    } else { html!{} }}
                                                    <span class="ingredient-name">{ing.name.clone()}</span>
                                                    {if let Some(notes) = &ing.notes {
                                                        html! { <span class="ingredient-notes">{"("}{notes}{")"}</span> }
                                                    } else { html!{} }}
                                                </li>
                                            }
                                        }).collect::<Html>()}
                                    </ul>
                                </div>
                            </div>
                        </div>

                        <div class="steps-section">
                            <div class="card page-enter">
                                <div class="card-body">
                                    <h2 class="section-title flex items-center gap-2 mb-4">
                                        <svg width="1.25rem" height="1.25rem" fill="none" stroke="var(--primary)" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"></path>
                                        </svg>
                                        {t("instructions", lang)}
                                    </h2>
                                    <ol class="step-list">
                                        {if let Some(arr) = r.steps.as_array() {
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
                                                let is_completed = if idx < (*completed_steps).len() { (*completed_steps)[idx] } else { false };

                                                html! {
                                                    <li class={classes!("step-item", is_completed.then_some("completed"))}
                                                        onclick={
                                                            let toggle_step = toggle_step.clone();
                                                            Callback::from(move |_| toggle_step.emit(idx))
                                                        }
                                                    >
                                                        <input
                                                            type="checkbox"
                                                            checked={is_completed}
                                                            onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                                                            onchange={
                                                                let toggle_step = toggle_step.clone();
                                                                Callback::from(move |_| toggle_step.emit(idx))
                                                            }
                                                            class="step-checkbox"
                                                        />
                                                        <span class="step-number">{idx + 1}</span>
                                                        <div class="step-content">
                                                            <p class="step-text">{step_text}</p>
                                                        </div>
                                                    </li>
                                                }
                                            }).collect::<Html>()
                                        } else {
                                            html! {}
                                        }}
                                    </ol>
                                </div>
                            </div>
                        </div>

                        {if let Some(notes) = &r.notes {
                            html! {
                                <div class="notes-section page-enter">
                                    <h3 class="notes-title">{t("notes", lang)}</h3>
                                    <p class="notes-text">{notes}</p>
                                </div>
                            }
                        } else { html!{} }}

                        // Lightbox
                        {if *lightbox_open && !r.images.is_empty() {
                            let current_image = &r.images[*lightbox_index];
                            let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}",
                                r.id.unwrap_or(0), current_image.filename);
                            let current_num = *lightbox_index + 1;
                            let total_num = r.images.len();

                            html! {
                                <div class="lightbox" onclick={close_lightbox.clone()}>
                                    <button class="lightbox-close"
                                        onclick={Callback::from(move |e: MouseEvent| {
                                            e.stop_propagation();
                                            close_lightbox.emit(e);
                                        })}
                                    >
                                        <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                        </svg>
                                    </button>

                                    {if r.images.len() > 1 {
                                        html! {
                                            <button class="lightbox-nav lightbox-nav-prev"
                                                onclick={Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    prev_image.emit(e);
                                                })}
                                            >
                                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                                                </svg>
                                            </button>
                                        }
                                    } else { html!{} }}

                                    <div onclick={Callback::from(|e: MouseEvent| e.stop_propagation())} style="text-align: center;">
                                        <img src={image_url} alt={current_image.alt.clone().unwrap_or_else(|| current_image.filename.clone())} />
                                        <div style="margin-top: var(--space-4); color: rgba(255,255,255,0.8);">
                                            <span class="text-sm">{format!("{} / {}", current_num, total_num)}</span>
                                        </div>
                                    </div>

                                    {if r.images.len() > 1 {
                                        html! {
                                            <button class="lightbox-nav lightbox-nav-next"
                                                onclick={Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    next_image.emit(e);
                                                })}
                                            >
                                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                                </svg>
                                            </button>
                                        }
                                    } else { html!{} }}
                                </div>
                            }
                        } else { html!{} }}

                        // Image Gallery (if more than 1 image)
                        {if r.images.len() > 1 {
                            html! {
                                <div class="card page-enter">
                                    <div class="card-body">
                                        <h2 class="section-title flex items-center gap-2 mb-4">
                                            <svg width="1.25rem" height="1.25rem" fill="none" stroke="var(--primary)" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                                            </svg>
                                            {t("gallery", lang)}
                                        </h2>
                                        <div class="gallery-thumbs">
                                            { for r.images.iter().enumerate().map(|(idx, image)| {
                                                let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}",
                                                    r.id.unwrap_or(0), image.filename);
                                                let is_primary = image.is_primary.unwrap_or(false);
                                                let open = open_lightbox.clone();

                                                html! {
                                                    <div class="relative" onclick={Callback::from(move |_| open.emit(idx))}>
                                                        <img src={image_url} alt={image.alt.clone().unwrap_or_else(|| image.filename.clone())} class={classes!("gallery-thumb", is_primary.then_some("active"))} />
                                                    </div>
                                                }
                                            }) }
                                        </div>
                                    </div>
                                </div>
                            }
                        } else { html!{} }}
                    </>
                }
            } else if let Some(e) = &*error {
                html! {
                    <div class="alert alert-error">
                        <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <div class="alert-content">
                            <div>{e}</div>
                            <button onclick={handle_back} class="btn btn-primary btn-sm mt-2">{t("back_to_recipes", lang)}</button>
                        </div>
                    </div>
                }
            } else {
                html! {
                    <div class="spinner"><div class="spinner-circle"></div></div>
                }
            }}
        </div>
    }
}
