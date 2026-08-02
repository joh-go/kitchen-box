use yew::prelude::*;
use yew::{platform::spawn_local, Callback, Properties};
use web_sys::{Event, HtmlSelectElement, HtmlInputElement, Blob, Url, HtmlAnchorElement};
use wasm_bindgen::JsCast;
use shared_types::{ImportResult, Recipe, RecipesExport};
use home_hub_shared::components::Modal;
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

#[derive(Clone, PartialEq)]
enum ImportState {
    Idle,
    Previewing { data: RecipesExport, file_name: String, selected: Vec<bool> },
    Done(ImportResult),
    Error(String),
}

fn download_json(data: &str, filename: &str) {
    if let Some(window) = web_sys::window() {
        let document = window.document().unwrap();
        let array = js_sys::Array::new();
        array.push(&wasm_bindgen::JsValue::from_str(data));
        if let Ok(blob) = Blob::new_with_str_sequence(&array) {
            if let Ok(url) = Url::create_object_url_with_blob(&blob) {
                if let Ok(anchor) = document.create_element("a") {
                    let anchor: HtmlAnchorElement = anchor.dyn_into().unwrap();
                    anchor.set_href(&url);
                    anchor.set_download(filename);
                    anchor.click();
                    let _ = Url::revoke_object_url(&url);
                }
            }
        }
    }
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
    let import_state = use_state(|| ImportState::Idle);
    let import_loading = use_state(|| false);
    let file_input_ref = use_node_ref();

    let on_add = props.on_add.clone();
    let delete_confirm = use_state(|| None::<(i32, String)>);

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
        let delete_confirm = delete_confirm.clone();
        Callback::from(move |id: i32| {
            let recipes = recipes.clone();
            let delete_confirm = delete_confirm.clone();
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
                delete_confirm.set(None);
            });
        })
    };

    html! {
        <>
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
                if is_logged_in {
                    html!{
                        <div class="import-export-bar page-enter">
                            <button
                                class="btn btn-secondary btn-sm"
                                onclick={
                                    let import_state = import_state.clone();
                                    Callback::from(move |_e: yew::MouseEvent| {
                                        let import_state = import_state.clone();
                                        import_state.set(ImportState::Idle);
                                        spawn_local(async move {
                                            match api::export_recipes().await {
                                                Ok(export_data) => {
                                                    let json = serde_json::to_string_pretty(&export_data).unwrap_or_default();
                                                    download_json(&json, "kitchenbox-recipes.json");
                                                }
                                                Err(e) => {
                                                    import_state.set(ImportState::Error(e));
                                                }
                                            }
                                        });
                                    })
                                }
                            >
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                                </svg>
                                {t("export_recipes", lang)}
                            </button>

                            <button
                                class="btn btn-secondary btn-sm"
                                onclick={
                                    let file_input_ref = file_input_ref.clone();
                                    Callback::from(move |_e: yew::MouseEvent| {
                                        if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                                            input.click();
                                        }
                                    })
                                }
                            >
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                                </svg>
                                {t("import_recipes", lang)}
                            </button>

                            <input
                                ref={file_input_ref}
                                type="file"
                                accept=".json"
                                style="display: none;"
                                onchange={
                                    let import_state = import_state.clone();
                                    Callback::from(move |e: Event| {
                                        let target = e.target().unwrap();
                                        let input: HtmlInputElement = target.dyn_into().unwrap();
                                        let files = input.files();
                                        let import_state = import_state.clone();
                                        if let Some(file_list) = files {
                                            if let Some(file) = file_list.get(0) {
                                                let file_name = file.name();
                                                let reader = web_sys::FileReader::new().unwrap();
                                                let import_state_clone = import_state.clone();
                                                let onload = wasm_bindgen::closure::Closure::once(Box::new(move |event: web_sys::Event| {
                                                    let target = event.target().unwrap();
                                                    let reader = target.dyn_into::<web_sys::FileReader>().unwrap();
                                                    let result = reader.result().unwrap();
                                                    let text = result.as_string().unwrap_or_default();
                                                match serde_json::from_str::<RecipesExport>(&text) {
                                                    Ok(data) => {
                                                        let count = data.recipes.len();
                                                        import_state_clone.set(ImportState::Previewing {
                                                            selected: vec![true; count],
                                                            data,
                                                            file_name,
                                                        });
                                                    }
                                                        Err(e) => {
                                                            import_state_clone.set(ImportState::Error(format!("Invalid file: {}", e)));
                                                        }
                                                    }
                                                }));
                                                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                                                onload.forget();
                                                reader.read_as_text(&file).unwrap();
                                            }
                                        }
                                        input.set_value("");
                                    })
                                }
                            />
                        </div>
                    }
                } else {
                    html!{}
                }
            }

            {
                match &*import_state {
                    ImportState::Previewing { data, file_name, selected } => {
                        let recipe_count = data.recipes.len();
                        let selected_count = selected.iter().filter(|&&s| s).count();
                        let all_selected = selected.iter().all(|&s| s);
                        let all_cats: Vec<String> = data.recipes.iter()
                            .flat_map(|r| r.categories.clone())
                            .collect::<std::collections::HashSet<_>>()
                            .into_iter()
                            .collect();
                        let import_data = data.clone();
                        html!{
                            <div class="import-preview page-enter">
                                <h3>{t("import_preview_title", lang)}</h3>
                                <p>{ format!("{}: {}", t("file", lang), file_name) }</p>
                                <p>{ format!("{} {} {}", recipe_count, t("recipes_count", lang), t("found", lang)) }</p>
                                <div class="import-preview-select-all">
                                    <label class="checkbox-label">
                                        <input
                                            type="checkbox"
                                            checked={all_selected}
                                            onchange={
                                                let import_state = import_state.clone();
                                                let data = data.clone();
                                                let file_name = file_name.clone();
                                                Callback::from(move |e: Event| {
                                                    let checked = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().checked();
                                                    let count = data.recipes.len();
                                                    import_state.set(ImportState::Previewing {
                                                        data: data.clone(),
                                                        file_name: file_name.clone(),
                                                        selected: vec![checked; count],
                                                    });
                                                })
                                            }
                                        />
                                        <span>{if all_selected { t("deselect_all", lang) } else { t("select_all", lang) }}</span>
                                    </label>
                                    <span class="text-sm text-muted">{ format!("{}/{} {} {}", selected_count, recipe_count, t("recipes_count", lang).to_lowercase(), t("selected", lang)) }</span>
                                </div>
                                { if !all_cats.is_empty() {
                                    html!{
                                        <div class="import-preview-cats">
                                            <strong>{format!("{}:", t("categories", lang))}</strong>
                                            { for all_cats.iter().map(|c| html!{ <span class="tag tag-new">{c}</span> }) }
                                        </div>
                                    }
                                } else { html!{} } }
                                <div class="import-preview-recipes">
                                    { for data.recipes.iter().enumerate().map(|(i, r)| {
                                        let cats = r.categories.join(", ");
                                        let checked = selected.get(i).copied().unwrap_or(true);
                                        html!{
                                            <div class="import-preview-recipe">
                                                <input
                                                    type="checkbox"
                                                    checked={checked}
                                                    class="import-checkbox"
                                                    onchange={
                                                        let import_state = import_state.clone();
                                                        let data = data.clone();
                                                        let file_name = file_name.clone();
                                                        let selected_snapshot = selected.clone();
                                                        Callback::from(move |_e: Event| {
                                                            let mut updated = selected_snapshot.clone();
                                                            updated[i] = !updated[i];
                                                            import_state.set(ImportState::Previewing {
                                                                data: data.clone(),
                                                                file_name: file_name.clone(),
                                                                selected: updated,
                                                            });
                                                        })
                                                    }
                                                />
                                                <div class="import-preview-recipe-info">
                                                    <strong>{&r.title}</strong>
                                                    { if !r.categories.is_empty() {
                                                        html!{ <span class="text-sm text-muted">{ format!("({})", cats) }</span> }
                                                    } else { html!{} } }
                                                </div>
                                            </div>
                                        }
                                    }) }
                                </div>
                                <div class="import-preview-actions">
                                    <button
                                        class="btn btn-primary btn-sm"
                                        disabled={*import_loading || selected_count == 0}
                                        onclick={
                                            let import_state = import_state.clone();
                                            let import_loading = import_loading.clone();
                                            let recipes = recipes.clone();
                                            let selected = selected.clone();
                                            Callback::from(move |_e: yew::MouseEvent| {
                                                let import_state = import_state.clone();
                                                let import_loading = import_loading.clone();
                                                let recipes = recipes.clone();
                                                let selected = selected.clone();
                                                let mut payload = import_data.clone();
                                                payload.recipes = payload.recipes.iter().enumerate()
                                                    .filter(|(i, _)| selected.get(*i).copied().unwrap_or(false))
                                                    .map(|(_, r)| r.clone())
                                                    .collect();
                                                import_loading.set(true);
                                                spawn_local(async move {
                                                    match api::import_recipes(&payload).await {
                                                        Ok(result) => {
                                                            import_state.set(ImportState::Done(result));
                                                            import_loading.set(false);
                                                            spawn_local(async move {
                                                                if let Ok(list) = api::get_recipes().await {
                                                                    recipes.set(list);
                                                                }
                                                            });
                                                        }
                                                        Err(e) => {
                                                            import_state.set(ImportState::Error(e));
                                                            import_loading.set(false);
                                                        }
                                                    }
                                                });
                                            })
                                        }
                                    >
                                        {
                                            if *import_loading {
                                                t("importing", lang)
                                            } else {
                                                if selected_count == 1 {
                                                    t("import_confirm_one", lang)
                                                } else {
                                                    format!("{} {} {}", t("import_confirm", lang), selected_count, t("recipes_count", lang).to_lowercase())
                                                }
                                            }
                                        }
                                    </button>
                                    <button
                                        class="btn btn-secondary btn-sm"
                                        disabled={*import_loading}
                                        onclick={
                                            let import_state = import_state.clone();
                                            Callback::from(move |_e: yew::MouseEvent| {
                                                import_state.set(ImportState::Idle);
                                            })
                                        }
                                    >
                                        {t("cancel", lang)}
                                    </button>
                                </div>
                            </div>
                        }
                    }
                    ImportState::Done(result) => {
                        html!{
                            <div class="import-result page-enter">
                                <div class={if result.errors.is_empty() { "alert alert-success" } else { "alert alert-warning" }}>
                                    <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        { if result.errors.is_empty() {
                                            html!{ <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path> }
                                        } else {
                                            html!{ <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path> }
                                        } }
                                    </svg>
                                    <div class="alert-content">
                                        <div class="alert-title">{t("import_complete", lang)}</div>
                                        <div>{ format!("{}: {}, {}: {}", t("imported", lang), result.created, t("skipped", lang), result.skipped) }</div>
                                        { for result.errors.iter().map(|e| html!{ <div class="text-sm text-muted">{e}</div> }) }
                                    </div>
                                </div>
                                <button
                                    class="btn btn-secondary btn-sm"
                                    onclick={
                                        let import_state = import_state.clone();
                                        Callback::from(move |_e: yew::MouseEvent| {
                                            import_state.set(ImportState::Idle);
                                        })
                                    }
                                >
                                    {t("close", lang)}
                                </button>
                            </div>
                        }
                    }
                    ImportState::Error(e) => {
                        html!{
                            <div class="import-result page-enter">
                                <div class="alert alert-error">
                                    <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                    <div class="alert-content">
                                        <div class="alert-title">{t("error_loading", lang)}</div>
                                        <div>{e}</div>
                                    </div>
                                </div>
                                <button
                                    class="btn btn-secondary btn-sm"
                                    onclick={
                                        let import_state = import_state.clone();
                                        Callback::from(move |_e: yew::MouseEvent| {
                                            import_state.set(ImportState::Idle);
                                        })
                                    }
                                >
                                    {t("close", lang)}
                                </button>
                            </div>
                        }
                    }
                    ImportState::Idle => html!{},
                }
            }

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
                                let image_url = format!("/uploads/recipes/{}/{}",
                                    r.id.unwrap_or(0), primary_image.filename);
                                html! {
                                    <img
                                        src={image_url}
                                        alt={primary_image.alt.clone().unwrap_or_else(|| r.title.clone())}
                                        class="recipe-card-image"
                                    />
                                }
                            } else if let Some(first_image) = r.images.first() {
                                let image_url = format!("/uploads/recipes/{}/{}",
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
                                    <svg class="recipe-card-image-placeholder" viewBox="0 0 400 192" xmlns="http://www.w3.org/2000/svg">
                                        <rect width="400" height="192" fill="none"/>
                                        <g transform="translate(200, 100)" fill="none" stroke="#d4c5b9" stroke-linecap="round" stroke-linejoin="round" opacity="0.55">
                                            <ellipse cx="0" cy="14" rx="48" ry="12" fill="#ebe3da" stroke-width="1.5"/>
                                            <ellipse cx="0" cy="10" rx="42" ry="10" fill="#f2ece3" stroke-width="1.5"/>
                                            <ellipse cx="0" cy="-24" rx="46" ry="18" fill="#ebe3da" stroke-width="1.2"/>
                                            <path d="M-46-24v-8a46 18 0 0192 0v8" fill="#e0d5c6" stroke-width="1.2"/>
                                            <path d="M-18-32c-2-8-8-18-16-22 8-2 16 2 18 8" stroke-width="1.4"/>
                                            <path d="M18-32c2-8 8-18 16-22-8-2-16 2-18 8" stroke-width="1.4"/>
                                            <circle cx="-28" cy="-8" r="2.5" fill="#d4c5b9"/>
                                            <circle cx="28" cy="-8" r="2.5" fill="#d4c5b9"/>
                                            <circle cx="0" cy="-14" r="2" fill="#d4c5b9"/>
                                            <circle cx="-10" cy="-16" r="1.8" fill="#d4c5b9"/>
                                            <circle cx="10" cy="-18" r="1.8" fill="#d4c5b9"/>
                                            <circle cx="-6" cy="-4" r="1.5" fill="#d4c5b9"/>
                                            <circle cx="6" cy="-6" r="1.5" fill="#d4c5b9"/>
                                        </g>
                                    </svg>
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
                                                onclick={
                                                    let delete_confirm = delete_confirm.clone();
                                                    let title = r.title.clone();
                                                    Callback::from(move |_| {
                                                        delete_confirm.set(Some((id, title.clone())));
                                                    })
                                                }
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

        {
            if let Some((confirm_id, confirm_title)) = delete_confirm.as_ref() {
                let id = *confirm_id;
                let title = confirm_title.clone();
                html!{
                    <Modal
                        title={t("delete_recipe_confirm", lang)}
                        show={true}
                        sm={true}
                        on_close={Callback::from({
                            let delete_confirm = delete_confirm.clone();
                            move |_| delete_confirm.set(None)
                        })}
                    >
                        <p>{ format!("{} \"{}\"?", t("delete_recipe_message", lang), title) }</p>
                        <div style="display: flex; gap: var(--space-3); margin-top: var(--space-4); justify-content: flex-end;">
                            <button
                                class="btn btn-secondary btn-sm"
                                onclick={
                                    let delete_confirm = delete_confirm.clone();
                                    Callback::from(move |_| delete_confirm.set(None))
                                }
                            >
                                {t("cancel", lang)}
                            </button>
                            <button
                                class="btn btn-danger btn-sm"
                                onclick={on_delete.reform(move |_| id)}
                            >
                                {t("delete", lang)}
                            </button>
                        </div>
                    </Modal>
                }
            } else {
                html!{}
            }
        }
        </>
    }
}
