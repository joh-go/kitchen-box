use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;

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

#[derive(Clone, Debug, PartialEq)]
pub enum RecipeAction {
    None,
    Delete(i32),
}

#[function_component(AdminRecipesPage)]
pub fn admin_recipes_page() -> Html {
    let recipes = use_state(|| Vec::<AdminRecipe>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let action = use_state(|| RecipeAction::None);
    let selected_recipe = use_state(|| None::<AdminRecipe>);

    // Load recipes
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
                            loading.set(false);
                        } else {
                            error.set(Some("Failed to parse recipes data".to_string()));
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

    let on_recipe_click = {
        let action = action.clone();
        let selected_recipe = selected_recipe.clone();
        Callback::from(move |recipe: AdminRecipe| {
            // Toggle selection - if clicking same recipe, deselect
            if let Some(ref selected) = *selected_recipe {
                if selected.id == recipe.id {
                    selected_recipe.set(None);
                    action.set(RecipeAction::None);
                } else {
                    selected_recipe.set(Some(recipe.clone()));
                    action.set(RecipeAction::None);
                }
            } else {
                selected_recipe.set(Some(recipe.clone()));
                action.set(RecipeAction::None);
            }
        })
    };

    let on_delete_recipe = {
        let recipes = recipes.clone();
        let action = action.clone();
        Callback::from(move |recipe_id: i32| {
            action.set(RecipeAction::Delete(recipe_id));
        })
    };

    // Handle delete action
    {
        let recipes = recipes.clone();
        let action = action.clone();
        
        use_effect_with(action.clone(), move |action| {
            if let RecipeAction::Delete(recipe_id) = **action {
                let recipes = recipes.clone();
                let action = action.clone();
                
                spawn_local(async move {
                    match api::delete_admin_recipe(recipe_id).await {
                        Ok(_) => {
                            // Remove recipe from list
                            let current_recipes = (*recipes).clone();
                            let updated_recipes: Vec<AdminRecipe> = current_recipes.into_iter().filter(|r| r.id != Some(recipe_id)).collect();
                            recipes.set(updated_recipes);
                            action.set(RecipeAction::None);
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("Failed to delete recipe: {}", e).into());
                        }
                    }
                });
            }
            || ()
        });
    }

    html! {
        <div class="space-y-6">
            // Header
            <div class="animate-fade-in">
                <div class="flex items-center justify-between">
                    <div>
                        <h1 class="text-2xl sm:text-3xl font-bold text-slate-800 dark:text-slate-200">
                            {"Recipe Management"}
                        </h1>
                        <p class="text-slate-500 dark:text-slate-400 mt-1">
                            {"Manage all recipes in the system"}
                        </p>
                    </div>
                </div>
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
                        // Recipes Table
                        <div class="glass rounded-2xl shadow-lg border border-emerald-100 dark:border-slate-700 overflow-hidden animate-fade-in">
                            <div class="overflow-x-auto">
                                <table class="w-full">
                                    <thead class="bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider">
                                                {"Recipe"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider hidden lg:table-cell">
                                                {"Author"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider hidden lg:table-cell">
                                                {"Status"}
                                            </th>
                                            <th class="px-6 py-3 text-right text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider">
                                                {"Actions"}
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-slate-200 dark:divide-slate-700">
                                        {for recipes.iter().map(|recipe| {
                                            let on_delete = on_delete_recipe.clone();
                                            let on_click = on_recipe_click.clone();
                                            let recipe_clone = recipe.clone();
                                            let recipe_id = recipe.id.unwrap_or(0);
                                            let is_selected = selected_recipe.as_ref().and_then(|r| r.id).unwrap_or(0) == recipe_id;
                                            
                                            html! {
                                                <>
                                                    <tr class="hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors">
                                                        <td class="px-6 py-4 whitespace-nowrap" onclick={Callback::from(move |_| {
                                                            on_click.emit(recipe_clone.clone());
                                                        })}>
                                                            <div class="flex items-center cursor-pointer">
                                                                <div class="w-8 h-8 bg-emerald-100 dark:bg-emerald-900 rounded-lg flex items-center justify-center mr-3">
                                                                    <span class="text-emerald-600 dark:text-emerald-400 text-sm font-medium">
                                                                        {&recipe.title.chars().next().unwrap_or('R').to_uppercase().to_string()}
                                                                    </span>
                                                                </div>
                                                                <div>
                                                                    <div class="text-sm font-medium text-slate-900 dark:text-slate-100">
                                                                        {&recipe.title}
                                                                    </div>
                                                                    {if is_selected {
                                                                        html! {
                                                                            <div class="text-xs text-slate-500 dark:text-slate-400 mt-1">
                                                                                {recipe.short_description.as_ref().unwrap_or(&"No description".to_string())}
                                                                            </div>
                                                                        }
                                                                    } else {
                                                                        html! {}
                                                                    }}
                                                                </div>
                                                            </div>
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap hidden lg:table-cell">
                                                            <div class="text-sm text-slate-600 dark:text-slate-400">
                                                                {recipe.author_name.as_ref().unwrap_or(&"Unknown".to_string())}
                                                            </div>
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap hidden lg:table-cell">
                                                            {if recipe.is_public {
                                                                html! {
                                                                    <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-emerald-100 text-emerald-800 dark:bg-emerald-900 dark:text-emerald-300">
                                                                        {"Public"}
                                                                    </span>
                                                                }
                                                            } else {
                                                                html! {
                                                                    <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-slate-100 text-slate-800 dark:bg-slate-700 dark:text-slate-300">
                                                                        {"Private"}
                                                                    </span>
                                                                }
                                                            }}
                                                        </td>
                                                        <td class="px-2 py-4 whitespace-nowrap text-right text-sm font-medium lg:px-6">
                                                            <button 
                                                                onclick={Callback::from(move |_| on_delete.emit(recipe_id))}
                                                                class="text-red-600 dark:text-red-400 hover:text-red-900 dark:hover:text-red-300 p-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                                                                title="Delete Recipe"
                                                            >
                                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6M4 7h16"></path>
                                                                </svg>
                                                            </button>
                                                        </td>
                                                    </tr>
                                                    // Expanded row with recipe details
                                                    {if is_selected {
                                                        html! {
                                                            <tr class="bg-slate-50 dark:bg-slate-800">
                                                                <td colspan="4" class="px-6 py-4">
                                                                    <div class="text-sm text-slate-600 dark:text-slate-400 space-y-2">
                                                                        <div>
                                                                            <span class="font-medium">{"Description: "}</span>
                                                                            {recipe.short_description.as_ref().unwrap_or(&"No description available".to_string())}
                                                                        </div>
                                                                        <div>
                                                                            <span class="font-medium">{"Author: "}</span>
                                                                            {recipe.author_name.as_ref().unwrap_or(&"Unknown".to_string())}
                                                                            {if let Some(ref email) = recipe.author_email {
                                                                                html! {
                                                    <span class="text-slate-500 dark:text-slate-500">{" ("}{email}{")"}</span>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                                                        </div>
                                                                        <div>
                                                                            <span class="font-medium">{"Status: "}</span>
                                                                            {if recipe.is_public {
                                                                                html! {
                                                                                    <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-emerald-100 text-emerald-800 dark:bg-emerald-900 dark:text-emerald-300 lg:hidden">
                                                                                        {"Public"}
                                                                                    </span>
                                                                                }
                                                                            } else {
                                                                                html! {
                                                                                    <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-slate-100 text-slate-800 dark:bg-slate-700 dark:text-slate-300 lg:hidden">
                                                                                        {"Private"}
                                                                                    </span>
                                                                                }
                                                                            }}
                                                                        </div>
                                                                        <div>
                                                                            <span class="font-medium">{"Created: "}</span>
                                                                            {&recipe.created_at}
                                                                        </div>
                                                                        <div>
                                                                            <span class="font-medium">{"Updated: "}</span>
                                                                            {&recipe.updated_at}
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
