use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_json::json;
use crate::api;

#[derive(Clone, Debug, PartialEq)]
pub struct AdminCategory {
    pub id: i32,
    pub name: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CategoryAction {
    None,
    Delete(i32),
}

#[function_component(AdminCategoriesPage)]
pub fn admin_categories_page() -> Html {
    let categories = use_state(|| Vec::<AdminCategory>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let action = use_state(|| CategoryAction::None);
    let selected_category = use_state(|| None::<AdminCategory>);
    let new_category_name = use_state(|| String::new());
    let creating = use_state(|| false);

    // Load categories
    {
        let categories = categories.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::get_admin_categories().await {
                    Ok(response) => {
                        if let Some(categories_data) = response.get("categories").and_then(|c| c.as_array()) {
                            let parsed_categories: Vec<AdminCategory> = categories_data.iter().filter_map(|category| {
                                Some(AdminCategory {
                                    id: category.get("id")?.as_i64()? as i32,
                                    name: category.get("name")?.as_str()?.to_string(),
                                    created_at: category.get("created_at").and_then(|c| c.as_str()).unwrap_or("").to_string(),
                                })
                            }).collect();
                            
                            categories.set(parsed_categories);
                            loading.set(false);
                        } else {
                            error.set(Some("Failed to parse categories data".to_string()));
                            loading.set(false);
                        }
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

    let on_category_click = {
        let action = action.clone();
        let selected_category = selected_category.clone();
        Callback::from(move |category: AdminCategory| {
            // Toggle selection - if clicking same category, deselect
            if let Some(ref selected) = *selected_category {
                if selected.id == category.id {
                    selected_category.set(None);
                    action.set(CategoryAction::None);
                } else {
                    selected_category.set(Some(category.clone()));
                    action.set(CategoryAction::None);
                }
            } else {
                selected_category.set(Some(category.clone()));
                action.set(CategoryAction::None);
            }
        })
    };

    let on_delete_category = {
        let action = action.clone();
        Callback::from(move |category_id: i32| {
            action.set(CategoryAction::Delete(category_id));
        })
    };

    // Handle delete action
    {
        let categories = categories.clone();
        let action = action.clone();
        
        use_effect_with(action.clone(), move |action| {
            if let CategoryAction::Delete(category_id) = **action {
                let categories = categories.clone();
                let action = action.clone();
                
                spawn_local(async move {
                    match api::delete_admin_category(category_id).await {
                        Ok(_) => {
                            // Remove category from list
                            let current_categories = (*categories).clone();
                            let updated_categories: Vec<AdminCategory> = current_categories.into_iter().filter(|c| c.id != category_id).collect();
                            categories.set(updated_categories);
                            action.set(CategoryAction::None);
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("Failed to delete category: {}", e).into());
                        }
                    }
                });
            }
            || ()
        });
    }

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
        <div class="space-y-6">
            // Header
            <div class="animate-fade-in">
                <div class="flex items-center justify-between">
                    <div>
                        <h1 class="text-2xl sm:text-3xl font-bold text-slate-800 dark:text-slate-200">
                            {"Category Management"}
                        </h1>
                        <p class="text-slate-500 dark:text-slate-400 mt-1">
                            {"Manage all categories in the system"}
                        </p>
                    </div>
                </div>
            </div>

            // Create Category Form
            <div class="glass rounded-2xl shadow-lg border border-emerald-100 dark:border-slate-700 p-6 animate-fade-in">
                <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4">
                    {"Add New Category"}
                </h2>
                <form onsubmit={on_create_category} class="flex flex-col sm:flex-row gap-3">
                    <input
                        type="text"
                        placeholder="Enter category name..."
                        value={(*new_category_name).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                            new_category_name.set(input.value());
                        })}
                        class="flex-1 px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                    />
                    <button
                        type="submit"
                        disabled={*creating}
                        class="bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed text-white font-medium py-2 px-6 rounded-lg transition-all transform hover:scale-105 shadow-lg flex items-center justify-center gap-2"
                    >
                        if *creating {
                            <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                            {"Creating..."}
                        } else {
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                            </svg>
                            {"Add Category"}
                        }
                    </button>
                </form>
            </div>

            // Loading State
            {if *loading {
                html! {
                    <div class="flex justify-center py-12">
                        <div class="w-8 h-8 border-4 border-emerald-600 border-t-transparent rounded-full animate-spin"></div>
                    </div>
                }
            } else if let Some(ref error_msg) = *error {
                html! {
                    <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-600 dark:text-red-400 px-4 py-3 rounded-lg">
                        {error_msg}
                    </div>
                }
            } else {
                html! {
                    <>
                        // Categories Table
                        <div class="glass rounded-2xl shadow-lg border border-emerald-100 dark:border-slate-700 overflow-hidden animate-fade-in">
                            <div class="overflow-x-auto">
                                <table class="w-full">
                                    <thead class="bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider">
                                                {"Category"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider hidden lg:table-cell">
                                                {"Created"}
                                            </th>
                                            <th class="px-2 py-3 text-right text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider lg:px-6">
                                                {"Actions"}
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-slate-200 dark:divide-slate-700">
                                        {for categories.iter().map(|category| {
                                            let on_delete = on_delete_category.clone();
                                            let on_click = on_category_click.clone();
                                            let category_clone = category.clone();
                                            let category_id = category.id;
                                            let is_selected = selected_category.as_ref().map(|c| c.id) == Some(category_id);
                                            
                                            html! {
                                                <>
                                                    <tr class="hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors">
                                                        <td class="px-6 py-4 whitespace-nowrap" onclick={Callback::from(move |_| {
                                                            on_click.emit(category_clone.clone());
                                                        })}>
                                                            <div class="flex items-center cursor-pointer">
                                                                <div class="w-8 h-8 bg-emerald-100 dark:bg-emerald-900 rounded-lg flex items-center justify-center mr-3">
                                                                    <span class="text-emerald-600 dark:text-emerald-400 text-sm font-medium">
                                                                        {&category.name.chars().next().unwrap_or('C').to_uppercase().to_string()}
                                                                    </span>
                                                                </div>
                                                                <div>
                                                                    <div class="text-sm font-medium text-slate-900 dark:text-slate-100">
                                                                        {&category.name}
                                                                    </div>
                                                                    <div class="text-xs text-slate-500 dark:text-slate-400 lg:hidden">
                                                                        {&category.created_at}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap hidden lg:table-cell">
                                                            <div class="text-sm text-slate-600 dark:text-slate-400">
                                                                {&category.created_at}
                                                            </div>
                                                        </td>
                                                        <td class="px-2 py-4 whitespace-nowrap text-right text-sm font-medium lg:px-6">
                                                            <button 
                                                                onclick={Callback::from(move |_| on_delete.emit(category_id))}
                                                                class="text-red-600 dark:text-red-400 hover:text-red-900 dark:hover:text-red-300 p-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                                                                title="Delete Category"
                                                            >
                                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6M4 7h16"></path>
                                                                </svg>
                                                            </button>
                                                        </td>
                                                    </tr>
                                                    // Expanded row with category details
                                                    {if is_selected {
                                                        html! {
                                                            <tr class="bg-slate-50 dark:bg-slate-800">
                                                                <td colspan="3" class="px-6 py-4">
                                                                    <div class="text-sm text-slate-600 dark:text-slate-400 space-y-2">
                                                                        <div>
                                                                            <span class="font-medium">{"Name: "}</span>
                                                                            {&category.name}
                                                                        </div>
                                                                        <div>
                                                                            <span class="font-medium">{"Created: "}</span>
                                                                            {&category.created_at}
                                                                        </div>
                                                                        <div>
                                                                            <span class="font-medium">{"ID: "}</span>
                                                                            {category.id}
                                                                        </div>
                                                                    </div>
                                                                </td>
                                                            </tr>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </>
                                            }
                                        })}
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </>
                }
            }}
        </div>
    }
}
