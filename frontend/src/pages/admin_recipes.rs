use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use home_hub_shared::icons::{Icon, IconComponent};
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;

#[derive(Clone, Debug, PartialEq)]
pub struct AdminRecipe {
    pub id: Option<i32>,
    pub title: String,
    pub short_description: Option<String>,
    pub author_id: Option<i32>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub is_public: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[function_component(AdminRecipesPage)]
pub fn admin_recipes_page() -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let recipes = use_state(|| Vec::<AdminRecipe>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let selected_recipe = use_state(|| None::<AdminRecipe>);

    {
        let recipes = recipes.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::get_admin_recipes().await {
                    Ok(response) => {
                        if let Some(recipes_data) = response.get("recipes").and_then(|r| r.as_array()) {
                            let parsed_recipes: Vec<AdminRecipe> = recipes_data.iter().filter_map(|recipe| {
                                Some(AdminRecipe {
                                    id: recipe.get("id")?.as_i64().map(|i| i as i32),
                                    title: recipe.get("title")?.as_str()?.to_string(),
                                    short_description: recipe.get("short_description").and_then(|d| d.as_str()).map(|s| s.to_string()),
                                    author_id: recipe.get("author_id").and_then(|a| a.as_i64()).map(|i| i as i32),
                                    author_name: recipe.get("author_name").and_then(|n| n.as_str()).map(|s| s.to_string()),
                                    author_email: recipe.get("author_email").and_then(|e| e.as_str()).map(|s| s.to_string()),
                                    is_public: recipe.get("is_public").and_then(|p| p.as_bool()).unwrap_or(false),
                                    created_at: recipe.get("created_at").and_then(|c| c.as_str()).unwrap_or("").to_string(),
                                    updated_at: recipe.get("updated_at").and_then(|u| u.as_str()).unwrap_or("").to_string(),
                                })
                            }).collect();
                            recipes.set(parsed_recipes);
                        } else {
                            error.set(Some(t("failed_parse_recipes", lang).to_string()));
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

    let on_delete_recipe = {
        let recipes = recipes.clone();
        Callback::from(move |recipe_id: i32| {
            let recipes = recipes.clone();
            spawn_local(async move {
                match api::delete_admin_recipe(recipe_id).await {
                    Ok(_) => {
                        let updated: Vec<AdminRecipe> = (*recipes).clone().into_iter().filter(|r| r.id != Some(recipe_id)).collect();
                        recipes.set(updated);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Failed to delete recipe: {}", e).into());
                    }
                }
            });
        })
    };

    html! {
        <div class="page-enter">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h1 class="section-title">{t("recipe_management_title", lang)}</h1>
                    <p class="text-muted">{t("manage_all_recipes_system", lang)}</p>
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
                                    <th>{t("recipe_column", lang)}</th>
                                    <th class="hide-mobile">{t("author_column", lang)}</th>
                                    <th class="hide-mobile">{t("status_column", lang)}</th>
                                    <th class="text-right">{t("actions_column", lang)}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for (*recipes).iter().map(|recipe| {
                                    let on_delete = on_delete_recipe.clone();
                                    let recipe_id = recipe.id.unwrap_or(0);
                                    let is_selected = selected_recipe.as_ref().and_then(|r| r.id).unwrap_or(0) == recipe_id;
                                    let on_click = {
                                        let selected_recipe = selected_recipe.clone();
                                        let r = recipe.clone();
                                        Callback::from(move |_| {
                                            if let Some(ref selected) = *selected_recipe {
                                                if selected.id == r.id {
                                                    selected_recipe.set(None);
                                                } else {
                                                    selected_recipe.set(Some(r.clone()));
                                                }
                                            } else {
                                                selected_recipe.set(Some(r.clone()));
                                            }
                                        })
                                    };

                                    html! {
                                        <>
                                            <tr class={if is_selected { "row-selected" } else { "" }}>
                                                <td onclick={on_click}>
                                                    <div class="flex items-center gap-3">
                                                        <div class="avatar avatar-primary">
                                                            {&recipe.title.chars().next().unwrap_or('R').to_uppercase().to_string()}
                                                        </div>
                                                        <div>
                                                            <div class="text-sm font-medium">{&recipe.title}</div>
                                                            <div class="text-xs text-muted hide-desktop">{recipe.author_name.as_ref().unwrap_or(&"Unknown".to_string())}</div>
                                                        </div>
                                                    </div>
                                                </td>
                                                <td class="hide-mobile"><span class="text-sm text-muted">{recipe.author_name.as_ref().unwrap_or(&"Unknown".to_string())}</span></td>
                                                <td class="hide-mobile">
                                                    {if recipe.is_public {
                                                        html! { <span class="badge badge-success">{t("public", lang)}</span> }
                                                    } else {
                                                        html! { <span class="badge">{t("private", lang)}</span> }
                                                    }}
                                                </td>
                                                <td class="text-right">
                                                    <button onclick={Callback::from(move |_| on_delete.emit(recipe_id))} class="btn btn-sm btn-danger">
                                                        <IconComponent kind={Icon::Delete} size={14} color="#ffffff" />
                                                        <span>{t("delete", lang)}</span>
                                                    </button>
                                                </td>
                                            </tr>
                                            {if is_selected {
                                                html! {
                                                    <tr class="table-expanded-row"><td colspan="4">
                                                        <div class="table-expanded-content">
                                                            <div class="table-expanded-label">{t("description_label", lang)}<span class="table-expanded-value">{recipe.short_description.as_ref().unwrap_or(&t("no_description_available", lang).to_string())}</span></div>
                                                            <div class="table-expanded-label">{t("author_label", lang)}<span class="table-expanded-value">{recipe.author_name.as_ref().unwrap_or(&t("unknown", lang).to_string())}{if let Some(ref email) = recipe.author_email { format!(" ({})", email) } else { String::new() }}</span></div>
                                                            <div class="table-expanded-label">{t("status_label", lang)}<span class="table-expanded-value">{if recipe.is_public { t("public", lang) } else { t("private", lang) }}</span></div>
                                                            <div class="table-expanded-label">{t("created_label", lang)}<span class="table-expanded-value">{&recipe.created_at}</span></div>
                                                            <div class="table-expanded-label">{t("updated_label", lang)}<span class="table-expanded-value">{&recipe.updated_at}</span></div>
                                                        </div>
                                                    </td></tr>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                        </>
                                    }
                                })}
                                {if (*recipes).is_empty() {
                                    html! { <tr><td colspan="4" class="table-empty">{t("no_recipes", lang)}</td></tr> }
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
