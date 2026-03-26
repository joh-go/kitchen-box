use yew::prelude::*;
use yew::{function_component, html, use_state, use_effect_with};
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct SettingsState {
    pub name: String,
    pub email: String,
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
    pub loading: bool,
    pub error: Option<String>,
    pub success: Option<String>,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            name: api::get_current_user_name().unwrap_or_default(),
            email: String::new(),
            current_password: String::new(),
            new_password: String::new(),
            confirm_password: String::new(),
            loading: false,
            error: None,
            success: None,
        }
    }
}

#[function_component(SettingsPage)]
pub fn settings() -> Html {
    let state = use_state(SettingsState::default);

    // Fetch current user data on page load
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            if api::is_logged_in() {
                spawn_local(async move {
                    match api::get_current_user().await {
                        Ok(user) => {
                            let name = user.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
                            let email = user.get("email").and_then(|e| e.as_str()).unwrap_or("").to_string();
                            state.set(SettingsState {
                                name,
                                email,
                                ..state.deref().clone()
                            });
                        }
                        Err(_) => {
                            // Silently fail - user can still enter data manually
                        }
                    }
                });
            }
            || ()
        });
    }

    let onsubmit = {
        let state = state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let state = state.clone();
            
            // Validate passwords match if changing password
            if !state.deref().new_password.is_empty() {
                if state.deref().new_password != state.deref().confirm_password {
                    state.set(SettingsState {
                        error: Some("New passwords do not match".to_string()),
                        success: None,
                        ..state.deref().clone()
                    });
                    return;
                }
                if state.deref().current_password.is_empty() {
                    state.set(SettingsState {
                        error: Some("Current password is required to change password".to_string()),
                        success: None,
                        ..state.deref().clone()
                    });
                    return;
                }
            }

            spawn_local(async move {
                state.set(SettingsState {
                    loading: true,
                    error: None,
                    success: None,
                    ..state.deref().clone()
                });

                let name = state.deref().name.clone();
                let email = state.deref().email.clone();
                let current_password = state.deref().current_password.clone();
                let new_password = state.deref().new_password.clone();

                match api::update_profile(&name, &email, &current_password, &new_password).await {
                    Ok(_) => {
                        // Update localStorage with new name
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("user_name", &name);
                            }
                        }
                        
                        state.set(SettingsState {
                            loading: false,
                            error: None,
                            success: Some("Profile updated successfully".to_string()),
                            current_password: String::new(),
                            new_password: String::new(),
                            confirm_password: String::new(),
                            ..state.deref().clone()
                        });
                    }
                    Err(e) => {
                        state.set(SettingsState {
                            loading: false,
                            error: Some(e),
                            success: None,
                            ..state.deref().clone()
                        });
                    }
                }
            });
        })
    };

    let oninput = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            
            if let Some(name) = input.get_attribute("name") {
                match name.as_str() {
                    "name" => {
                        state.set(SettingsState {
                            name: value,
                            ..state.deref().clone()
                        });
                    }
                    "email" => {
                        state.set(SettingsState {
                            email: value,
                            ..state.deref().clone()
                        });
                    }
                    "current_password" => {
                        state.set(SettingsState {
                            current_password: value,
                            ..state.deref().clone()
                        });
                    }
                    "new_password" => {
                        state.set(SettingsState {
                            new_password: value,
                            ..state.deref().clone()
                        });
                    }
                    "confirm_password" => {
                        state.set(SettingsState {
                            confirm_password: value,
                            ..state.deref().clone()
                        });
                    }
                    _ => {}
                }
            }
        })
    };

    html! {
        <div class="space-y-6">
            <div class="animate-fade-in">
                <h1 class="text-2xl sm:text-3xl font-bold text-slate-800 dark:text-slate-200">
                    {"Account Settings"}
                </h1>
                <p class="text-slate-500 dark:text-slate-400 mt-1">
                    {"Manage your profile and password"}
                </p>
            </div>

            <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
                <form class="space-y-6" onsubmit={onsubmit}>
                    {if let Some(ref error) = state.deref().error {
                        html! {
                            <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-600 dark:text-red-400 px-4 py-3 rounded-lg">
                                {error}
                            </div>
                        }
                    } else {
                        html! {}
                    }}

                    {if let Some(ref success) = state.deref().success {
                        html! {
                            <div class="bg-emerald-50 dark:bg-emerald-900/20 border border-emerald-200 dark:border-emerald-800 text-emerald-600 dark:text-emerald-400 px-4 py-3 rounded-lg">
                                {success}
                            </div>
                        }
                    } else {
                        html! {}
                    }}

                    <div>
                        <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4">
                            {"Profile Information"}
                        </h2>
                        
                        <div class="space-y-4">
                            <div>
                                <label for="name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {"Display Name"}
                                </label>
                                <input
                                    id="name"
                                    name="name"
                                    type="text"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder="Enter your display name"
                                    value={state.name.clone()}
                                    oninput={oninput.clone()}
                                />
                            </div>

                            <div>
                                <label for="email" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {"Email Address"}
                                </label>
                                <input
                                    id="email"
                                    name="email"
                                    type="email"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder="Enter your email"
                                    value={state.email.clone()}
                                    oninput={oninput.clone()}
                                />
                            </div>
                        </div>
                    </div>

                    <div class="border-t border-slate-200 dark:border-slate-700 pt-6">
                        <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4">
                            {"Change Password"}
                        </h2>
                        
                        <div class="space-y-4">
                            <div>
                                <label for="current_password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {"Current Password"}
                                </label>
                                <input
                                    id="current_password"
                                    name="current_password"
                                    type="password"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder="Enter current password"
                                    value={state.current_password.clone()}
                                    oninput={oninput.clone()}
                                />
                            </div>

                            <div>
                                <label for="new_password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {"New Password"}
                                </label>
                                <input
                                    id="new_password"
                                    name="new_password"
                                    type="password"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder="Enter new password"
                                    value={state.new_password.clone()}
                                    oninput={oninput.clone()}
                                />
                            </div>

                            <div>
                                <label for="confirm_password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {"Confirm New Password"}
                                </label>
                                <input
                                    id="confirm_password"
                                    name="confirm_password"
                                    type="password"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder="Confirm new password"
                                    value={state.confirm_password.clone()}
                                    oninput={oninput}
                                />
                            </div>
                        </div>
                    </div>

                    <div class="flex justify-end">
                        <button
                            type="submit"
                            disabled={state.loading}
                            class="touch-target btn-primary text-white px-6 py-2.5 rounded-lg font-medium flex items-center justify-center gap-2 transition-all duration-200 disabled:opacity-50"
                        >
                            {if state.loading {
                                html! {
                                    <>
                                        <svg class="animate-spin -ml-1 mr-2 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0h12c6.627 0 12 5.373 12v12c0 6.627-5.373 12-12h-4zm-1 1.465L9.465 15H15v-2h-4v-2h4v-2z"></path>
                                        </svg>
                                        {"Saving..."}
                                    </>
                                }
                            } else {
                                html! {"Save Changes"}
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
