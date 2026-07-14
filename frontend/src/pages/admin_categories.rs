use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_json::json;
use home_hub_shared::icons::{Icon, IconComponent};
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;

#[derive(Clone, Debug, PartialEq)]
pub struct AdminCategory {
    pub id: i32,
    pub name: String,
    pub created_at: String,
}

#[function_component(AdminCategoriesPage)]
pub fn admin_categories_page() -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let categories = use_state(|| Vec::<AdminCategory>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let selected_category = use_state(|| None::<AdminCategory>);
    let new_category_name = use_state(|| String::new());
    let creating = use_state(|| false);

    {
        let categories = categories.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::get_admin_categories().await {
                    Ok(response) => {
                        if let Some(categories_data) = response.get("categories").and_then(|c| c.as_array()) {
                            let parsed_categories: Vec<AdminCategory> = categories_data.iter().filter_map(|c| {
                                Some(AdminCategory {
                                    id: c.get("id")?.as_i64()? as i32,
                                    name: c.get("name")?.as_str()?.to_string(),
                                    created_at: c.get("created_at").and_then(|c| c.as_str()).unwrap_or("").to_string(),
                                })
                            }).collect();
                            categories.set(parsed_categories);
                        } else {
                            error.set(Some(t("failed_parse_categories", lang).to_string()));
                        }
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    let on_delete_category = {
        let categories = categories.clone();
        Callback::from(move |category_id: i32| {
            let categories = categories.clone();
            spawn_local(async move {
                match api::delete_admin_category(category_id).await {
                    Ok(_) => {
                        let updated: Vec<AdminCategory> = (*categories).clone().into_iter().filter(|c| c.id != category_id).collect();
                        categories.set(updated);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Failed to delete category: {}", e).into());
                    }
                }
            });
        })
    };

    let on_create_category = {
        let categories = categories.clone();
        let new_category_name = new_category_name.clone();
        let creating = creating.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = (*new_category_name).clone().trim().to_string();
            if name.is_empty() {
                return;
            }

            let categories = categories.clone();
            let new_category_name = new_category_name.clone();
            let creating = creating.clone();

            creating.set(true);
            spawn_local(async move {
                let category_data = json!({"name": name});
                match api::create_admin_category(category_data).await {
                    Ok(response) => {
                        if let Some(id) = response.get("id").and_then(|i| i.as_i64()) {
                            let category = AdminCategory {
                                id: id as i32,
                                name: response.get("name").and_then(|n| n.as_str()).unwrap_or(&name).to_string(),
                                created_at: response.get("created_at").and_then(|c| c.as_str()).unwrap_or("").to_string(),
                            };
                            let mut current = (*categories).clone();
                            current.push(category);
                            current.sort_by(|a, b| a.name.cmp(&b.name));
                            categories.set(current);
                        }
                        new_category_name.set(String::new());
                        creating.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Failed to create category: {}", e).into());
                        creating.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class="page-enter">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h1 class="section-title">{t("category_management_title", lang)}</h1>
                    <p class="text-muted">{t("manage_all_categories_system", lang)}</p>
                </div>
            </div>

            <div class="card mb-6">
                <div class="card-body">
                    <h2 class="section-title mb-4">{t("add_new_category", lang)}</h2>
                    <form onsubmit={on_create_category} class="flex gap-3">
                        <input
                            type="text"
                            placeholder={t("enter_category_name", lang)}
                            value={(*new_category_name).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                new_category_name.set(input.value());
                            })}
                            class="form-input flex-1"
                        />
                        <button type="submit" disabled={*creating} class="btn btn-primary">
                            {if *creating {
                                html! { <><span class="spinner-spin"></span> {t("creating_category", lang)}</> }
                            } else {
                                html! { {t("add_category_button", lang)} }
                            }}
                        </button>
                    </form>
                </div>
            </div>

            {if *loading {
                html! { <div class="spinner"><div class="spinner-circle"></div></div> }
            } else if let Some(ref error_msg) = *error {
                html! { <div class="alert alert-error"><div class="alert-content">{error_msg}</div></div> }
            } else {
                html! {
                    <div class="table-container card">
                        <table>
                            <thead>
                                <tr>
                                    <th>{t("category_column", lang)}</th>
                                    <th class="hide-mobile">{t("created_label", lang)}</th>
                                    <th class="text-right">{t("actions_column", lang)}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for (*categories).iter().map(|category| {
                                    let on_delete = on_delete_category.clone();
                                    let category_id = category.id;
                                    let is_selected = selected_category.as_ref().map(|c| c.id) == Some(category_id);
                                    let on_click = {
                                        let selected_category = selected_category.clone();
                                        let cat = category.clone();
                                        Callback::from(move |_| {
                                            if let Some(ref selected) = *selected_category {
                                                if selected.id == cat.id {
                                                    selected_category.set(None);
                                                } else {
                                                    selected_category.set(Some(cat.clone()));
                                                }
                                            } else {
                                                selected_category.set(Some(cat.clone()));
                                            }
                                        })
                                    };

                                    html! {
                                        <>
                                            <tr class={if is_selected { "row-selected" } else { "" }}>
                                                <td onclick={on_click}>
                                                    <div class="flex items-center gap-3">
                                                        <div class="avatar avatar-primary">
                                                            {&category.name.chars().next().unwrap_or('C').to_uppercase().to_string()}
                                                        </div>
                                                        <div>
                                                            <div class="text-sm font-medium">{&category.name}</div>
                                                            <div class="text-xs text-muted hide-desktop">{&category.created_at}</div>
                                                        </div>
                                                    </div>
                                                </td>
                                                <td class="hide-mobile"><span class="text-sm text-muted">{&category.created_at}</span></td>
                                                <td class="text-right">
                                                    <button onclick={Callback::from(move |_| on_delete.emit(category_id))} class="btn btn-sm btn-danger">
                                                        <IconComponent kind={Icon::Delete} size={14} color="#ffffff" />
                                                        <span>{t("delete", lang)}</span>
                                                    </button>
                                                </td>
                                            </tr>
                                            {if is_selected {
                                                html! {
                                                    <tr class="table-expanded-row"><td colspan="3">
                                                        <div class="table-expanded-content">
                                                            <div class="table-expanded-label">{t("name_label", lang)}<span class="table-expanded-value">{&category.name}</span></div>
                                                            <div class="table-expanded-label">{t("created_label", lang)}<span class="table-expanded-value">{&category.created_at}</span></div>
                                                            <div class="table-expanded-label">{t("id_label", lang)}<span class="table-expanded-value">{category.id}</span></div>
                                                        </div>
                                                    </td></tr>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                        </>
                                    }
                                })}
                                {if (*categories).is_empty() {
                                    html! { <tr><td colspan="3" class="table-empty">{t("no_categories", lang)}</td></tr> }
                                } else {
                                    html! {}
                                }}
                            </tbody>
                        </table>
                    </div>
                }
            }}
        </div>
    }
}
