use yew::prelude::*;
use yew::{function_component, html, use_state, use_effect_with};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use crate::api;

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: String,
}

#[derive(Clone, Debug)]
pub enum UserAction {
    None,
    Create,
    Edit(i32),
    Delete(i32),
}

#[function_component(AdminUsersPage)]
pub fn admin_users_page() -> Html {
    let users = use_state(|| Vec::<User>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let action = use_state(|| UserAction::None);
    let show_create_form = use_state(|| false);
    let selected_user = use_state(|| None::<User>);

    // Form state
    let form_name = use_state(|| String::new());
    let form_email = use_state(|| String::new());
    let form_password = use_state(|| String::new());
    let form_is_admin = use_state(|| false);
    let form_loading = use_state(|| false);
    let form_error = use_state(|| None::<String>);

    // Load users
    {
        let users = users.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::get_admin_users().await {
                    Ok(response) => {
                        if let Some(users_data) = response.get("users").and_then(|u| u.as_array()) {
                            let parsed_users: Vec<User> = users_data.iter().filter_map(|user| {
                                Some(User {
                                    id: user.get("id")?.as_i64()? as i32,
                                    name: user.get("name")?.as_str()?.to_string(),
                                    email: user.get("email")?.as_str()?.to_string(),
                                    is_admin: user.get("is_admin")?.as_bool().unwrap_or(false),
                                    created_at: user.get("created_at")?.as_str()?.to_string(),
                                })
                            }).collect();
                            
                            users.set(parsed_users);
                            loading.set(false);
                        } else {
                            error.set(Some("Failed to parse users data".to_string()));
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

    let on_create_user = {
        let show_create_form = show_create_form.clone();
        Callback::from(move |_| {
            show_create_form.set(true);
        })
    };

    let on_user_click = {
        let action = action.clone();
        let selected_user = selected_user.clone();
        Callback::from(move |user: User| {
            // Toggle selection - if clicking same user, deselect
            if let Some(ref selected) = *selected_user {
                if selected.id == user.id {
                    selected_user.set(None);
                    action.set(UserAction::None);
                } else {
                    selected_user.set(Some(user.clone()));
                    action.set(UserAction::None);
                }
            } else {
                selected_user.set(Some(user.clone()));
                action.set(UserAction::None);
            }
        })
    };

    let on_edit_user = {
        let action = action.clone();
        Callback::from(move |user_id: i32| {
            action.set(UserAction::Edit(user_id));
        })
    };

    let on_delete_user = {
        let action = action.clone();
        Callback::from(move |user_id: i32| {
            action.set(UserAction::Delete(user_id));
        })
    };

    let on_cancel_action = {
        let action = action.clone();
        let show_create_form = show_create_form.clone();
        let form_name = form_name.clone();
        let form_email = form_email.clone();
        let form_password = form_password.clone();
        let form_is_admin = form_is_admin.clone();
        let form_error = form_error.clone();
        
        Callback::from(move |_| {
            action.set(UserAction::None);
            show_create_form.set(false);
            // Reset form
            form_name.set(String::new());
            form_email.set(String::new());
            form_password.set(String::new());
            form_is_admin.set(false);
            form_error.set(None);
        })
    };

    let on_form_submit = {
        let form_name = form_name.clone();
        let form_email = form_email.clone();
        let form_password = form_password.clone();
        let form_is_admin = form_is_admin.clone();
        let form_loading = form_loading.clone();
        let form_error = form_error.clone();
        let users = users.clone();
        let action = action.clone();
        let show_create_form = show_create_form.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let name = (*form_name).clone();
            let email = (*form_email).clone();
            let password = (*form_password).clone();
            let is_admin = *form_is_admin;
            let current_action = (*action).clone();

            // Validation
            if name.trim().is_empty() {
                form_error.set(Some("Name is required".to_string()));
                return;
            }
            if email.trim().is_empty() {
                form_error.set(Some("Email is required".to_string()));
                return;
            }
            if password.trim().is_empty() && matches!(current_action, UserAction::Create) {
                form_error.set(Some("Password is required for new users".to_string()));
                return;
            }

            form_loading.set(true);
            form_error.set(None);

            let users_clone = users.clone();
            let action_clone = action.clone();
            let show_create_form_clone = show_create_form.clone();
            let form_loading_clone = form_loading.clone();
            let form_name_clone = form_name.clone();
            let form_email_clone = form_email.clone();
            let form_password_clone = form_password.clone();
            let form_is_admin_clone = form_is_admin.clone();
            let form_error_clone = form_error.clone();

            spawn_local(async move {
                let result = match current_action {
                    UserAction::Create => {
                        api::create_admin_user(&name, &email, &password, is_admin).await
                    }
                    UserAction::Edit(user_id) => {
                        if password.is_empty() {
                            api::update_admin_user(user_id, Some(&name), Some(&email), None, Some(is_admin)).await
                        } else {
                            api::update_admin_user(user_id, Some(&name), Some(&email), Some(&password), Some(is_admin)).await
                        }
                    }
                    _ => Err("Invalid action".to_string()),
                };

                match result {
                    Ok(_) => {
                        // Reload users list
                        match api::get_admin_users().await {
                            Ok(response) => {
                                if let Some(users_data) = response.get("users").and_then(|u| u.as_array()) {
                                    let parsed_users: Vec<User> = users_data.iter().filter_map(|user| {
                                        Some(User {
                                            id: user.get("id")?.as_i64()? as i32,
                                            name: user.get("name")?.as_str()?.to_string(),
                                            email: user.get("email")?.as_str()?.to_string(),
                                            is_admin: user.get("is_admin")?.as_bool().unwrap_or(false),
                                            created_at: user.get("created_at")?.as_str()?.to_string(),
                                        })
                                    }).collect();
                                    
                                    users_clone.set(parsed_users);
                                }
                            }
                            Err(_) => {}
                        }
                        
                        // Reset form and close
                        action_clone.set(UserAction::None);
                        show_create_form_clone.set(false);
                        form_name_clone.set(String::new());
                        form_email_clone.set(String::new());
                        form_password_clone.set(String::new());
                        form_is_admin_clone.set(false);
                        form_loading_clone.set(false);
                    }
                    Err(e) => {
                        form_error_clone.set(Some(e));
                        form_loading_clone.set(false);
                    }
                }
            });
        })
    };

    let on_confirm_delete = {
        let users = users.clone();
        let action = action.clone();
        let user_id = if let UserAction::Delete(id) = *action { id } else { 0 };

        Callback::from(move |_| {
            let users = users.clone();
            let action = action.clone();
            let user_id = user_id;

            spawn_local(async move {
                match api::delete_admin_user(user_id).await {
                    Ok(_) => {
                        // Remove user from list
                        let current_users = (*users).clone();
                        let updated_users: Vec<User> = current_users.into_iter().filter(|u| u.id != user_id).collect();
                        users.set(updated_users);
                        action.set(UserAction::None);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Failed to delete user: {}", e).into());
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
                            {"User Management"}
                        </h1>
                        <p class="text-slate-500 dark:text-slate-400 mt-1">
                            {"Manage user accounts and permissions"}
                        </p>
                    </div>
                    <button 
                        onclick={on_create_user}
                        class="touch-target btn-primary text-white px-4 py-2 rounded-lg font-medium flex items-center gap-2 transition-all duration-200"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                        </svg>
                        {"Add User"}
                    </button>
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
                        // Users Table
                        <div class="glass rounded-2xl shadow-lg border border-emerald-100 dark:border-slate-700 overflow-hidden animate-fade-in">
                            <div class="overflow-x-auto">
                                <table class="w-full">
                                    <thead class="bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider">
                                                {"User"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider hidden lg:table-cell">
                                                {"Role"}
                                            </th>
                                            <th class="px-6 py-3 text-right text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider">
                                                {"Actions"}
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-slate-200 dark:divide-slate-700">
                                        {for users.iter().map(|user| {
                                            let on_edit = on_edit_user.clone();
                                            let on_delete = on_delete_user.clone();
                                            let on_click = on_user_click.clone();
                                            let user_clone = user.clone();
                                            let user_id = user.id;
                                            let is_selected = selected_user.as_ref().map(|u| u.id).unwrap_or(0) == user_id;
                                            
                                            html! {
                                                <>
                                                    <tr class="hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors">
                                                        <td class="px-6 py-4 whitespace-nowrap" onclick={Callback::from(move |_| {
                            let user = user_clone.clone();
                            on_click.emit(user);
                        })}>
                                                            <div class="flex items-center cursor-pointer">
                                                                <div class="w-8 h-8 bg-emerald-100 dark:bg-emerald-900 rounded-full flex items-center justify-center mr-3">
                                                                    <span class="text-emerald-600 dark:text-emerald-400 text-sm font-medium">
                                                                        {&user.name.chars().next().unwrap_or('U').to_uppercase().to_string()}
                                                                    </span>
                                                                </div>
                                                                <div>
                                                                    <div class="text-sm font-medium text-slate-900 dark:text-slate-100">
                                                                        {&user.name}
                                                                    </div>
                                                                    {if is_selected {
                                                                        html! {
                                                                            <div class="text-xs text-slate-500 dark:text-slate-400 mt-1">
                                                                                {&user.email}
                                                                            </div>
                                                                        }
                                                                    } else {
                                                                        html! {}
                                                                    }}
                                                                </div>
                                                            </div>
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap hidden lg:table-cell">
                                                        {if user.is_admin {
                                                            html! {
                                                                <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-emerald-100 text-emerald-800 dark:bg-emerald-900 dark:text-emerald-300">
                                                                    {"Admin"}
                                                                </span>
                                                            }
                                                        } else {
                                                            html! {
                                                                <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-slate-100 text-slate-800 dark:bg-slate-700 dark:text-slate-300">
                                                                    {"User"}
                                                                </span>
                                                            }
                                                        }}
                                                    </td>
                                                    <td class="px-2 py-4 whitespace-nowrap text-right text-sm font-medium lg:px-6">
                                                        <button 
                                                            onclick={Callback::from(move |_| on_edit.emit(user_id))}
                                                            class="text-emerald-600 dark:text-emerald-400 hover:text-emerald-900 dark:hover:text-emerald-300 p-2 rounded-lg hover:bg-emerald-50 dark:hover:bg-emerald-900/20 transition-colors lg:mr-3"
                                                            title="Edit User"
                                                        >
                                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2H5a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-11h-1z"></path>
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 11l3 3L22 9l-3-3"></path>
                                                            </svg>
                                                        </button>
                                                        <button 
                                                            onclick={Callback::from(move |_| on_delete.emit(user_id))}
                                                            class="text-red-600 dark:text-red-400 hover:text-red-900 dark:hover:text-red-300 p-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                                                            title="Delete User"
                                                        >
                                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6M4 7h16"></path>
                                                            </svg>
                                                        </button>
                                                    </td>
                                                    </tr>
                                                    // Expanded row with email, created_at, and role
                                                    {if is_selected {
                                                        html! {
                                                            <tr class="bg-slate-50 dark:bg-slate-800">
                                                                <td colspan="3" class="px-6 py-4">
                                                                    <div class="text-sm text-slate-600 dark:text-slate-400 space-y-2">
                                                                        <div>
                                                                            <span class="font-medium">{"Email: "}</span>
                                                                            {&user.email}
                                                                        </div>
                                                                        <div>
                                                                            <span class="font-medium">{"Created: "}</span>
                                                                            {&user.created_at}
                                                                        </div>
                                                                        <div>
                                                                            {if user.is_admin {
                                                                                html! {
                                                                                    <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-emerald-100 text-emerald-800 dark:bg-emerald-900 dark:text-emerald-300 lg:hidden">
                                                                                        {"Admin"}
                                                                                    </span>
                                                                                }
                                                                            } else {
                                                                                html! {
                                                                                    <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-slate-100 text-slate-800 dark:bg-slate-700 dark:text-slate-300 lg:hidden">
                                                                                        {"User"}
                                                                                    </span>
                                                                                }
                                                                            }}
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

                        // Create/Edit Modal
                        {if *show_create_form || matches!(*action, UserAction::Edit(_)) {
                            html! {
                                <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                                    <div class="bg-white dark:bg-slate-800 rounded-2xl p-6 w-full max-w-md mx-4">
                                        <h2 class="text-xl font-semibold text-slate-800 dark:text-slate-200 mb-4">
                                            {if matches!(*action, UserAction::Edit(_)) { "Edit User" } else { "Create User" }}
                                        </h2>

                                        {if let Some(ref error_msg) = *form_error {
                                            html! {
                                                <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-600 dark:text-red-400 px-3 py-2 rounded-lg mb-4 text-sm">
                                                    {error_msg}
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }}

                                        <form class="space-y-4" onsubmit={on_form_submit}>
                                            <div>
                                                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                                                    {"Name"}
                                                </label>
                                                <input
                                                    type="text"
                                                    value={(*form_name).clone()}
                                                    oninput={Callback::from(move |e: yew::InputEvent| {
                                                        let input = e.target_unchecked_into::<HtmlInputElement>();
                                                        form_name.set(input.value());
                                                    })}
                                                    class="w-full px-3 py-2 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-emerald-500"
                                                    placeholder="Enter name"
                                                    required=true
                                                />
                                            </div>

                                            <div>
                                                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                                                    {"Email"}
                                                </label>
                                                <input
                                                    type="email"
                                                    value={(*form_email).clone()}
                                                    oninput={Callback::from(move |e: yew::InputEvent| {
                                                        let input = e.target_unchecked_into::<HtmlInputElement>();
                                                        form_email.set(input.value());
                                                    })}
                                                    class="w-full px-3 py-2 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-emerald-500"
                                                    placeholder="Enter email"
                                                    required=true
                                                />
                                            </div>

                                            <div>
                                                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                                                    {"Password"}
                                                </label>
                                                <input
                                                    type="password"
                                                    value={(*form_password).clone()}
                                                    oninput={Callback::from(move |e: yew::InputEvent| {
                                                        let input = e.target_unchecked_into::<HtmlInputElement>();
                                                        form_password.set(input.value());
                                                    })}
                                                    class="w-full px-3 py-2 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-emerald-500"
                                                    placeholder={if matches!(*action, UserAction::Edit(_)) { "Leave blank to keep current password" } else { "Enter password" }}
                                                    required={matches!(*action, UserAction::Create)}
                                                />
                                            </div>

                                            <div class="flex items-center">
                                                <input
                                                    type="checkbox"
                                                    id="is_admin"
                                                    checked={*form_is_admin}
                                                    onchange={Callback::from(move |e: yew::Event| {
                                                        let input = e.target_unchecked_into::<HtmlInputElement>();
                                                        form_is_admin.set(input.checked());
                                                    })}
                                                    class="w-4 h-4 text-emerald-600 bg-white dark:bg-slate-700 border-slate-300 dark:border-slate-600 rounded focus:ring-emerald-500"
                                                />
                                                <label for="is_admin" class="ml-2 text-sm text-slate-700 dark:text-slate-300">
                                                    {"Administrator"}
                                                </label>
                                            </div>

                                            <div class="flex justify-end space-x-3 pt-4">
                                                <button
                                                    type="button"
                                                    onclick={on_cancel_action.clone()}
                                                    class="px-4 py-2 text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-slate-700 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors"
                                                >
                                                    {"Cancel"}
                                                </button>
                                                <button
                                                    type="submit"
                                                    disabled={*form_loading}
                                                    class="px-4 py-2 bg-emerald-500 hover:bg-emerald-600 text-white rounded-lg transition-colors disabled:opacity-50"
                                                >
                                                    {if *form_loading { "Saving..." } else { if matches!(*action, UserAction::Edit(_)) { "Update" } else { "Create" } }}
                                                </button>
                                            </div>
                                        </form>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}

                        // Delete Confirmation Modal
                        {if let UserAction::Delete(user_id) = *action {
                            html! {
                                <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                                    <div class="bg-white dark:bg-slate-800 rounded-2xl p-6 w-full max-w-sm mx-4">
                                        <div class="text-center">
                                            <div class="w-12 h-12 bg-red-100 dark:bg-red-900 rounded-full flex items-center justify-center mx-auto mb-4">
                                                <svg class="w-6 h-6 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
                                                </svg>
                                            </div>
                                            <h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
                                                {"Delete User"}
                                            </h3>
                                            <p class="text-slate-600 dark:text-slate-400 mb-6">
                                                {"Are you sure you want to delete this user? This action cannot be undone."}
                                            </p>
                                            <div class="flex justify-center space-x-3">
                                                <button
                                                    onclick={on_cancel_action.clone()}
                                                    class="px-4 py-2 text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-slate-700 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors"
                                                >
                                                    {"Cancel"}
                                                </button>
                                                <button
                                                    onclick={on_confirm_delete}
                                                    class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors"
                                                >
                                                    {"Delete"}
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </>
                }
            }}
        </div>
    }
}
