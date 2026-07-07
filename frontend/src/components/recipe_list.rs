use yew::prelude::*;
use yew::{platform::spawn_local, Callback, Properties};
use web_sys::{Event, HtmlSelectElement};
use wasm_bindgen::JsCast;
use shared_types::{Recipe};
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_edit: Callback<Recipe>,
    pub on_view: Callback<i32>,
    pub on_add: Callback<()>,
    pub refresh: i32,
    pub search: String,
    pub on_search: Callback<String>,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &Props) -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let recipes = use_state(|| Vec::<Recipe>::new());
    let error = use_state(|| None::<String>);
    let categories = use_state(|| Vec::<shared_types::Category>::new());
    let selected_category = use_state(|| None as Option<i32>);
    let is_logged_in = api::is_logged_in();
    let current_user_id = api::get_current_user_id();
    let search = props.search.clone();

    let on_add = props.on_add.clone();

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

    {
        let categories = categories.clone();
        use_effect_with((), move |_| {
            let categories = categories.clone();
            spawn_local(async move {
                if let Ok(list) = api::get_categories().await {
                    categories.set(list);
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
        <div class="flex flex-col gap-6">
            <div class="page-enter">
                <div class="section-header">
                    <div>
                        <h1 class="page-title">{t("your_recipes", lang)}</h1>
                        <p class="text-sm text-muted">
                            { format!("{} {}", (*recipes).len(), t("delicious_recipes_count", lang)) }
                        </p>
                    </div>

                    <div class="filter-bar">
                        <label class="text-sm text-muted">{t("filter_by_category", lang)}</label>
                        <select
                            onchange={Callback::from({
                                let selected_category = selected_category.clone();
                                move |e: Event| {
                                    let v: String = e.target()
                                        .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                                        .map(|el: HtmlSelectElement| el.value())
                                        .unwrap_or_default();

                                    let new_selected = if v.is_empty() {
                                        None
                                    } else {
                                        v.parse::<i32>().ok()
                                    };

                                    selected_category.set(new_selected);
                                }
                            })}
                            class="form-select filter-select"
                        >
                            <option value="" selected=true>{ t("all_categories", lang) }</option>
                            { for (*categories).iter().map(|c| html!{
                                <option value={c.id.map(|id| id.to_string()).unwrap_or_default()}>{ &c.name }</option>
                            }) }
                        </select>
                    </div>

                    <div class="mobile-only w-full">
                        <div class="filter-search-inner">
                            <svg class="filter-search-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                            </svg>
                            <input
                                type="text"
                                placeholder={t("search_recipes_placeholder", lang)}
                                value={search.clone()}
                                oninput={
                                    let on_search = props.on_search.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                        on_search.emit(input.value());
                                    })
                                }
                                class="form-input"
                                style="padding-left: 2.5rem;"
                            />
                        </div>
                    </div>
                </div>
            </div>

            {
                if let Some(e) = &*error {
                    html!{
                        <div class="alert alert-error page-enter">
                            <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            <div class="alert-content">
                                <div class="alert-title">{t("error_loading_recipes", lang)}</div>
                                <div>{ e }</div>
                            </div>
                        </div>
                    }
                } else { html!{} }
            }

            <div class="recipe-grid">
                { for (*recipes).iter().filter(|r| {
                    let search_match = if search.is_empty() {
                        true
                    } else {
                        let q = search.to_lowercase();
                        r.title.to_lowercase().contains(&q) || r.short_description.clone().unwrap_or_default().to_lowercase().contains(&q)
                    };

                    let category_match = if let Some(selected_id) = *selected_category {
                        r.categories.iter().any(|c| c.id == Some(selected_id))
                    } else {
                        true
                    };

                    search_match && category_match
                }).map(|r| {
                    let id = r.id.unwrap_or_default();
                    let r_clone = r.clone();

                    let is_owned = if is_logged_in {
                        r.author_id.is_some() &&
                        current_user_id.map(|uid| uid == r.author_id.unwrap_or_default()).unwrap_or(false)
                    } else {
                        false
                    };

                    html!{
                        <div
                            class="recipe-card"
                            onclick={props.on_view.reform(move |_| id)}
                        >
                            {if let Some(primary_image) = r.images.iter().find(|img| img.is_primary == Some(true)) {
                                let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}",
                                    r.id.unwrap_or(0), primary_image.filename);
                                html! {
                                    <img
                                        src={image_url}
                                        alt={primary_image.alt.clone().unwrap_or_else(|| r.title.clone())}
                                        class="recipe-card-image"
                                    />
                                }
                            } else if let Some(first_image) = r.images.first() {
                                let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}",
                                    r.id.unwrap_or(0), first_image.filename);
                                html! {
                                    <img
                                        src={image_url}
                                        alt={first_image.alt.clone().unwrap_or_else(|| r.title.clone())}
                                        class="recipe-card-image"
                                    />
                                }
                            } else {
                                html! {
                                    <div class="recipe-card-image-placeholder">{"🍳"}</div>
                                }
                            }}

                            <div class="recipe-card-body">
                                <div class="flex items-start justify-between mb-4">
                                    <div class="flex-1">
                                        <h3 class="recipe-card-title">
                                            { &r.title }
                                        </h3>
                                        <p class="recipe-card-desc">
                                            { r.short_description.clone().unwrap_or_default() }
                                        </p>
                                    </div>

                                    if let Some(prep_time) = r.prep_minutes {
                                        <div class="recipe-card-time">
                                            <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                            </svg>
                                            { format!("{}m", prep_time) }
                                        </div>
                                    }
                                </div>

                                { if !r.categories.is_empty() {
                                    let mut seen_ids = Vec::new();
                                    let unique_categories: Vec<_> = r.categories.iter()
                                        .filter(|cat| {
                                            if let Some(id) = cat.id {
                                                if seen_ids.contains(&id) {
                                                    false
                                                } else {
                                                    seen_ids.push(id);
                                                    true
                                                }
                                            } else {
                                                true
                                            }
                                        })
                                        .take(3)
                                        .collect();
                                    let total_unique = r.categories.iter()
                                        .filter(|cat| {
                                            if let Some(id) = cat.id {
                                                seen_ids.contains(&id)
                                            } else {
                                                true
                                            }
                                        })
                                        .count();
                                    let remaining_count = total_unique.saturating_sub(3);

                                    html!{
                                        <div class="recipe-card-tags">
                                            { for unique_categories.iter().map(|cat| {
                                                html!{
                                                    <span class="tag">{ &cat.name }</span>
                                                }
                                            })}
                                            { if remaining_count > 0 {
                                                html!{
                                                    <span class="tag-more">{ format!("+{}", remaining_count) }</span>
                                                }
                                            } else {
                                                html!{}
                                            }}
                                        </div>
                                    }
                                } else { html!{} } }

                                {if is_owned {
                                    html! {
                                        <div class="recipe-card-actions" onclick={|e: yew::MouseEvent| e.stop_propagation()}>
                                            <button
                                                class="btn btn-primary btn-sm"
                                                onclick={props.on_edit.reform(move |_| r_clone.clone())}
                                            >
                                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                                </svg>
                                                {"Edit"}
                                            </button>
                                            <button
                                                class="btn btn-danger btn-sm"
                                                onclick={on_delete.reform(move |_| id)}
                                            >
                                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                                                </svg>
                                                {"Delete"}
                                            </button>
                                        </div>
                                    }
                                } else { html!{} }}
                            </div>
                        </div>
                    }
                }) }
            </div>

            { if (*recipes).is_empty() && error.is_none() {
                html!{
                    <div class="empty-state page-enter">
                        <svg class="empty-state-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                        </svg>
                        <h3>{t("no_recipes_yet", lang)}</h3>
                        <p>{t("start_building_collection", lang)}</p>
                        <button
                            onclick={Callback::from(move |_e: yew::MouseEvent| on_add.emit(()))}
                            class="btn btn-primary">
                            <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                            </svg>
                            {t("add_first_recipe", lang)}
                        </button>
                    </div>
                }
            } else { html!{} } }
        </div>
    }
}
